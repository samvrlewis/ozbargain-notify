'use strict';

const scrapeIt = require("scrape-it");
const AWS = require('aws-sdk');
const gcm = require('node-gcm');
const dynamoDb = new AWS.DynamoDB.DocumentClient();

const date_regex = /([\d]{2}\/[\d]{2}\/[\d]{4}) - ([\d]{2}:[\d]{2})/i;
const table_name = "deals";
const keys = require('./keys.js');

var sender = new gcm.Sender(keys.gcm);

var save_deal = function (deal) {
    var deal_to_save = deal;

    var params = {
        TableName: table_name,
        Key: {
            "id": deal_to_save.id
        },
        ExpressionAttributeValues: {
            ":v": deal_to_save.votes,
            ":d": deal_to_save.date,
            ":t": deal_to_save.title,
        },
        UpdateExpression: "SET created = if_not_exists(created, :d), \
                               votes=:v, \
                               title = if_not_exists(title, :t)"
    }

    dynamoDb.update(params, function (err, data) {
        if (err) {
            console.error("Unable to update item. Error JSON:", JSON.stringify(err, null, 2));
        } else {
            console.log("UpdateItem succeeded:", JSON.stringify(deal_to_save));
        }
    });
}

var notify_deal = function (deal, channel) {
    var message = new gcm.Message({
        data: {
            title: deal.title,
            notId: deal.id,
            url_intent: "https://www.ozbargain.com.au/node/" + deal.id,
            ledColor: [0, 0, 255, 0]
        }
    });

    sender.send(message, { topic: "/topics/" + channel }, function (err, response) {
        if (err) console.error(err);
        else console.log(response);
    });

}

var run_notify = function (deal, cb, err_back) {
    var categories = { "all_deals": 0, "over_50": 50, "over_100": 100 };

    var params = {
        TableName: "deals",
        KeyConditionExpression: "#id = :idValue",
        ExpressionAttributeNames: {
            "#id": "id"
        },
        ExpressionAttributeValues: {
            ":idValue": deal.id
        }
    };

    var old_votes = 0;
    var new_votes = deal.votes;
    var new_deal = true;
    var notified = false;

    dynamoDb.query(params, function (err, data) {
        if (err) {
            console.error("Unable to query. Error:", JSON.stringify(err, null, 2));
        } else {
            console.log("Query succeeded");
            data.Items.forEach(function (item) {
                console.log("Found " + JSON.stringify(item) + " for deal: " + JSON.stringify(deal));
                old_votes = item.votes;
                new_deal = false;
            });

            for (var category in categories) {
                var vote_threshold = categories[category];

                if ((category === "all_deals" && new_deal) || (old_votes < vote_threshold && new_votes >= vote_threshold)) {
                    notified = true;
                    console.log("Notifying " + category + " of " + deal.title);
                    notify_deal(deal, category);
                }
            }

            if (notified) {
                console.log("Saving " + deal.title);
                save_deal(deal);
            }
        }
    });
}


module.exports.get_deals = (cb, err_back) => {
    scrapeIt("http://www.ozbargain.com.au", {
        deals: {
            listItem: ".node-ozbdeal",
            data: {
                title: "h2",
                votes: {
                    selector: ".voteup > span",
                    convert: x => parseInt(x)
                },
                id: {
                    selector: ".n-deal",
                    attr: "data-nid"
                },
                date: {
                    selector: ".submitted",
                    how: "html",
                    convert: x => x.match(date_regex)[0]
                }
            }
        }

    }).then(page => {
        if (page.deals.length > 0) {
            console.info("Scraped: ")
            console.info(page.deals)
            cb(page.deals);
        } else {
            console.error("No deals found when scraping")
            err_back("No results");
        }
    }).catch(err => {
        console.error("Error getting deals: " + err)
        err_back(err);
    });
}

module.exports.persist_deals = (deals, cb, err_back) => {
    for (var i = 0; i < deals.length; i++) {
        run_notify(deals[i], cb, err_back);
    }
}


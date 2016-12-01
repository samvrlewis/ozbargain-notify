'use strict';

const deal_scraper = require("./deal_scraper.js");

module.exports.scrape = (event, context, callback) => {
  deal_scraper.get_deals(function(deals){
    deal_scraper.persist_deals(deals, function() {
      console.info("Successfully persisted deals");
    }, function(error) {
      console.error("Error persisting deals");
    });
  }, function(error)
  {
    console.error("Error getting deals");
  });
};

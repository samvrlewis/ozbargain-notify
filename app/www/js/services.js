angular.module('starter.services', [])

  .factory('notifications', function () {

    var unsubscribeTopics = function (push, topics, callback) {
      if (topics.length == 0) {
        callback("Success");
      } else {
          push.unsubscribe(topics.pop(), function (data) {
          console.log("Success unsub")
          console.log(data);
          unsubscribeTopics(push, topics, callback);
        }, function (data) {
          console.log("Error unsubbing")
          console.log(data);
          unsubscribeTopics(push, topics, callback);
        });
      }
    }

    return {
      unsubscribeFromTopics: unsubscribeTopics
    };
  });

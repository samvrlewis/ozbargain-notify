angular.module('starter.services', [])

  .factory('notifications', function () {

    var unsubscribeTopics = function (push, topics, callback, err_callback) {
          push.unregister(function (data) {
          console.log("Success unsub");
          console.log(data);
          callback("Success");
        }, function (data) {
          console.log("Error unsubbing");
          console.log(data);
          err_callback("Fail");
        }, topics);
    };
  
    return {
      unsubscribeFromTopics: unsubscribeTopics
    };
  });

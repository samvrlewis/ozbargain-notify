angular.module('starter.controllers', ['ionic'])

  .controller('SettingsCtrl', function ($scope, notifications) {
    $scope.settings = {
      vibration: true,
      sound: true,
      led: true
    };

    $scope.channels = ["all_deals", "over_50", "over_100"];

    $scope.notification_choice = { choice: localStorage.getItem("notification_choice") || "all_deals" };

    $scope.notificationChange = function () {
      console.log($scope.notification_choice);
      localStorage.setItem("notification_choice", $scope.notification_choice.choice);

      /* Provide a copy of the channels to unsub from */
      notifications.unsubscribeFromTopics($scope.push, $scope.channels.slice(0), function () {
        $scope.push.subscribe($scope.notification_choice.choice, function () {
          console.log("Sucessfully subscribed")
        }, function (error) {
          alert("Error registering for topic");
        });
      });
    }

    ionic.Platform.ready(function () {
      console.log(ionic.Platform.platform());
      $scope.push = PushNotification.init({
        android: {
          senderID: "987757989249",
          sound: false,
          vibrate: false,
          icon: "notify"
        }
      });

      $scope.push.on('registration', function (data) {
        console.log(data);
      });

      $scope.push.on('notification', function (data) {
        console.log(data);
      });

      if(!localStorage.getItem("notification_choice"))
      {
        /* pretend that the user has chosen the default */
        $scope.notificationChange();
      }
    });

    if (ionic.Platform.platform() !== "linux") {

    }
  });

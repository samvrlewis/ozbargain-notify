angular.module('starter.controllers', ['ionic'])

  .controller('SettingsCtrl', function ($scope, notifications, $ionicLoading, $ionicPopup) {
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
      $ionicLoading.show();

      /* Provide a copy of the channels to unsub from */
      notifications.unsubscribeFromTopics($scope.push, $scope.channels.slice(0), function () {
        $scope.push.subscribe($scope.notification_choice.choice, function () {
          $ionicPopup.alert({title: "Successfully subscribed"});
          $ionicLoading.hide();
        }, function (error) {
          $ionicPopup.alert({title: "Error registering for topic"});
          $ionicLoading.hide();
        });
      }, function(error) {
          $ionicPopup.alert({title: "Error unsubscribing from topics prior to subscribing"});
          $ionicLoading.hide();
      });
    }

    ionic.Platform.ready(function () {
      console.log(ionic.Platform.platform());
      $ionicLoading.show();

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
        $ionicLoading.hide();

        /* pretend that the user has chosen the default */
        $scope.notificationChange();
      });

      $scope.push.on('notification', function (data) {
        console.log(data);
      });
      
      
    });

    if (ionic.Platform.platform() !== "linux") {

    }
  });

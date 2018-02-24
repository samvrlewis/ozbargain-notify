App publishing
cordova build --release android
./home/sam/android-sdk-linux/build-tools/25.0.0/zipalign -v -p 4 /home/sam/development/ozbargain-notify/app/platforms/android/build/outputs/apk/android-release-unsigned.apk
 /home/sam/development/ozbargain-notify/app/platforms/android/build/outputs/apk/android-release-unsigned-aligned.apk

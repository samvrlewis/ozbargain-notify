#!/usr/bin/env node

var fs = require('fs');
var path = require('path');
var mkdirp = require('mkdirp');

var inputFolder = path.join(__dirname, '../../', 'resources/android/push_icon/');
var outputFolder = path.join(__dirname, '../../', '/platforms/android/res/');

console.log('------------------------------------------------------------------------------------------');
console.log("Running hook: "+path.basename(process.env.CORDOVA_HOOK));
console.log('------------------------------------------------------------------------------------------');

fs.readdir(inputFolder, function(err, list) {
    list.forEach(function(file){
        if (file.indexOf('drawable') === 0) {
            var destFolder = file.replace('-icon.png','');
            mkdirp(outputFolder + destFolder, function (err) {
                fs.createReadStream(inputFolder+file)
                .pipe(fs.createWriteStream(outputFolder + destFolder + '/notify.png'));
                console.log('# ' + file + ' --> ' + destFolder);
            });
            
        }
    });
    console.log('-----------------------------------------------------------------------------------------');
});
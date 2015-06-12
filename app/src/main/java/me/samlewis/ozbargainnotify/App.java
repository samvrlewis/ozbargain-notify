package me.samlewis.ozbargainnotify;

import android.app.Application;
import com.parse.Parse;

public class App extends Application {

    @Override
    public void onCreate() {
        super.onCreate();

        Parse.initialize(this, "FgsHhePRFNHSMoru4AEnkRugmeywstIJWTQ5VUG8", "GyldjicsH9MyuecP5KdBHLTtEyzc1FyWNKgCNzgh");

    }
} 
package me.samlewis.ozbargainnotify;

import android.app.Activity;
import android.content.Context;
import android.content.SharedPreferences;
import android.os.Bundle;
import android.view.View;
import android.widget.RadioButton;
import android.widget.RadioGroup;
import android.widget.Toast;

import com.parse.ParseAnalytics;
import com.parse.ParseException;
import com.parse.ParseInstallation;
import com.parse.ParsePush;
import com.parse.SaveCallback;


import android.util.Log;

import java.util.List;

public class SettingsActivity extends Activity {

    public static final String PREFS_NAME = "subscribed_channel";
    public static final String SETTINGS_NAME = "channel_int";

    /** Called when the activity is first created. */
    public void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_settings);


        if (isFirstTime())
        {
            //subscribe to all deals
            subscribeTo("all_deals");

            SharedPreferences settings = getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE);
            SharedPreferences.Editor editorRG = settings.edit();
            editorRG.putInt(SETTINGS_NAME, R.id.all_deals);
            editorRG.commit();

        }

        RadioGroup rg = (RadioGroup) findViewById(R.id.deals_rg);
        SharedPreferences settings = getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE);
        int preference = settings.getInt(SETTINGS_NAME, R.id.all_deals);

        rg.check(preference);
        ParseAnalytics.trackAppOpenedInBackground(getIntent());
    }

    private Boolean firstTime = null;
    /**
     * Checks if the user is opening the app for the first time.
     * Note that this method should be placed inside an activity and it can be called multiple times.
     * @return boolean
     */
    private boolean isFirstTime() {
        if (firstTime == null) {
            SharedPreferences mPreferences = this.getSharedPreferences("first_time", Context.MODE_PRIVATE);
            firstTime = mPreferences.getBoolean("firstTime", true);
            if (firstTime) {
                SharedPreferences.Editor editor = mPreferences.edit();
                editor.putBoolean("firstTime", false);
                editor.commit();
            }
        }
        return firstTime;
    }

    public void onRadioButtonClicked(View view) {
        // Is the button now checked?
        boolean checked = ((RadioButton) view).isChecked();
        CharSequence text = "";
        boolean updated = false;

        SharedPreferences settings = getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE);
        SharedPreferences.Editor editor = settings.edit();

        switch (view.getId()) {
            case R.id.all_deals:
                if (checked) {
                    updated = subscribeTo("all_deals");
                    text = "Will notify for all deals.";
                    editor.putInt(SETTINGS_NAME, R.id.all_deals);
                }
                break;
            case R.id.over_50:
                if (checked) {
                    updated = subscribeTo("over_50");
                    text = "Will notify for deals with a score greater than 50.";
                    editor.putInt(SETTINGS_NAME, R.id.over_50);
                }
                break;

            case R.id.over_100:
                if (checked) {
                    updated = subscribeTo("over_100");
                    text = "Will notify for deals with a score greater than 100.";
                    editor.putInt(SETTINGS_NAME, R.id.over_100);
                }
                break;
        }

        if (updated) {
            editor.commit();
            Context context = getApplicationContext();
            Toast.makeText(context, text, Toast.LENGTH_SHORT).show();
        }
    }

    public void unsubfromall(List<String> subscribedChannels)
    {
        for (final String channel : subscribedChannels)
        {
            ParsePush.unsubscribeInBackground(channel, new SaveCallback() {
                @Override
                public void done(ParseException e) {
                    if (e == null) {
                        //Log.d("com.parse.push", "successfully usubscribed to" + channel);
                    } else {
                        //Log.e("com.parse.push", "failed to unsubscribe to" + channel, e);
                    }
                }
            });
        }

    }
    //Returns true if subscribed or false if didn't because already subscribed
    public boolean subscribeTo(final String channel)
    {
        List<String> subscribedChannels = ParseInstallation.getCurrentInstallation().getList("channels");

        if (subscribedChannels != null && subscribedChannels.contains(channel) && subscribedChannels.size() == 1)
        {
            //already subscribed, don't need to do anything
            return false;
        }

        if (subscribedChannels != null) {
            unsubfromall(subscribedChannels);
        }
        Log.d("testing12", "subscribed to " + channel);
        ParsePush.subscribeInBackground(channel, new SaveCallback() {
            @Override
            public void done(ParseException e) {
                if (e == null) {
                    //Log.d("testing12", "successfully subscribed to " + channel);
                } else {
                    //Log.e("testing12", "failed to subscribe to " + channel, e);
                }
            }
        });
        return true;
    }
}

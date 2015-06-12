package me.samlewis.ozbargainnotify;

import android.app.Notification;
import android.app.NotificationManager;
import android.app.PendingIntent;
import android.content.Context;
import android.content.Intent;
import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.graphics.Color;
import android.renderscript.RenderScript;
import android.support.v4.app.NotificationCompat;
import android.app.Activity;
import com.parse.ParsePushBroadcastReceiver;

import org.json.JSONObject;
import org.json.JSONException;
import android.net.Uri;

public class PushReceiver extends ParsePushBroadcastReceiver {

    public static final String PARSE_DATA_KEY = "com.parse.Data";

    @Override
    protected Notification getNotification(Context context, Intent intent) {
        // deactivate standard notification
        return null;
    }

    @Override
    protected void onPushOpen(Context context, Intent intent) {
        // Implement
    }

    @Override
    protected void onPushReceive(Context context, Intent intent) {
        JSONObject data = getDataFromIntent(intent);
        String title = null;
        String url = null;
        try {
            title = data.getString("alert");
            url = data.getString("uri");
        } catch (JSONException e) {
            return;
        }

        NotificationManager notificationManager =
                (NotificationManager) context.getSystemService(Context.NOTIFICATION_SERVICE);

        NotificationCompat.Builder builder = new NotificationCompat.Builder(context);
        builder.setContentTitle("New OzBargain Deal");
        builder.setContentText(title);
        builder.setSmallIcon(R.drawable.ic_shopping);

        //in the future this could be from the deal image
        //Bitmap largeIcon = BitmapFactory.decodeResource(context.getResources(), R.drawable.ic_shopping);
        //builder.setLargeIcon(largeIcon);
        builder.setAutoCancel(true);
        builder.setColor(Color.argb(0, 226, 70, 55));
        //builder.setPriority(1); //high priority
        builder.setDefaults(Notification.DEFAULT_SOUND | Notification.DEFAULT_LIGHTS | Notification.DEFAULT_VIBRATE);
        //builder.setLights(Color.argb(0, 255, 255, 255)); //white
        Intent resultIntent = new Intent(Intent.ACTION_VIEW);
        resultIntent.setData(Uri.parse(url));

        PendingIntent pending = PendingIntent.getActivity(context, 0, resultIntent, PendingIntent.FLAG_ONE_SHOT);
        builder.setContentIntent(pending);

        // TODO: Set a sound and allow this to be customised
        //builder.setSound(soundUri);

        notificationManager.notify("MyTag", 0, builder.build());
    }

    private JSONObject getDataFromIntent(Intent intent) {
        JSONObject data = null;
        try {
            data = new JSONObject(intent.getExtras().getString(PARSE_DATA_KEY));
        } catch (JSONException e) {
            // Json was not readable...
        }
        return data;
    }
}
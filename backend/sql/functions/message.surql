DEFINE FUNCTION message::send($from: uuid, $channel: uuid, $content: string, $reply_to: option<uuid>) {

    LET $content_id = rand::uuid::v4();
    CREATE MessageStringContent CONTENT {
        id: $content_id,
        content: $content
    };

    LET $message_id = rand::uuid::v4();
    CREATE Message CONTENT {
        id: $message_id,
        channel: Channel:channel,
        edited: false,
        sent: time::now(),
        reply_to: $reply_to,
    };

    RELATE Message:message_id -> HasContent -> MessageStringContent:content_id;
    RELATE Entity:from -> SentMessage -> Message:message_id;
}

DEFINE FUNCTION message::edit($user: uuid, $message: uuid, $content: string) {
    LET $id = (SELECT id 
        FROM Entity:user
        ->SentMessage
        ->(Message WHERE id = $message)
        ->HasContent
        ->MessageStringContent)[0];

    UPDATE MessageStringContent SET content = $content WHERE id = $id.id;
    UPDATE Message:message SET edited = true;
}
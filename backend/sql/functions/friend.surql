DEFINE FUNCTION friend::request($from: uuid, $to: uuid) {
    RELATE Entity:from -> FriendRequest -> Entity:to
    content {
        created = time::now()
    };
}

DEFINE FUNCTION friend::accept($from: uuid, $to: uuid) {

    -- check if there is a friend request

    IF !HasFriendRequest WHERE out = Entity:from AND in = Entity:to {
        RETURN;
    }

    LET $fsid: uuid = rand::uuid::v4();
    CREATE Friendship SET id = $fsid, channel_id = fn::channel::new(), since = time::now();
    RELATE Entity:from -> HasFriendship -> Friendship:fsid CONTENT { nickname = Entity:from.displayname };
    RELATE Entity:to -> HasFriendship -> Friendship:fsid CONTENT { nickname = Entity:to.displayname };
}

DEFINE FUNCTION friend::reject($from: uuid, $to: uuid) {
    DELETE Entity:from -> FriendRequest WHERE out = Entity:to
}

DEFINE FUNCTION friend::remove($from: uuid, $to: uuid) {
    DELETE Entity:from -> HasFriendship -> Friendship WHERE <- HasFriendship.out <- Entity:to
}
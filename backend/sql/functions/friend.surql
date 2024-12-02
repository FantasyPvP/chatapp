DEFINE FUNCTION friend::request($from: uuid, $to: uuid) {
    CREATE FriendRequest SET
        in = Entity:from,
        out = Entity:to,
        created = time::now(),
}

DEFINE FUNCTION friend::accept($request: record<FriendRequest>) {

    LET $fsid = uuid::new();

    CREATE Friendship SET
        dm_channel = channel::new(),
        id = $fsid,
        since = time::now(),   

    CREATE HasFriendShip SET
        in = Entity:request.in,
        out = Friendship:fsid,
        nickname = Entity:request.out.displayname,

    CREATE HasFriendShip SET
        in = Entity:request.out,
        out = Friendship:fsid,
        nickname = Entity:request.in.displayname,
}
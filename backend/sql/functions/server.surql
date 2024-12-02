DEFINE FUNCTION server::join($server: uuid, $entity: uuid) {
    LET $user = (SELECT displayname FROM Entity WHERE id = $entity);
    RELATE Entity:entity -> HasServer -> Server:server CONTENT {
        joined: time::now(),
        nickname: $user.displayname
    };
}

DEFINE FUNCTION server::leave($server: uuid, $entity: uuid) {
    DELETE HasServer WHERE out = Server:server AND in = Entity:entity;
    DELETE HasRole WHERE in = Entity:entity AND ->Role.server = Server:server;
}

DEFINE FUNCTION server::new($name: string, $creator: uuid) {
    LET $id = rand::uuid::v4();

    CREATE Server CONTENT {
        created: time::now(),
        name: $name,
        owner: $creator,
        id: $id,
        icon_uri: fn::icon::default(),
    };

    fn::server::join($id, $creator)
}
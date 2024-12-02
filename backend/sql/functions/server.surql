DEFINE FUNCTION server::join($server_id: uuid, $entity_id: uuid) {

    LET $user = (SELECT displayname FROM Entity WHERE id = $entity_id)[0];

    CREATE HasServer SET
        in = Entity:entity_id,
        out = Server:server_id,
        nickname = $user.displayname,
        permissions = [],
        joined = time::now(),
}
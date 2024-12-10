DEFINE FUNCTION user::human::new(
    $email: string, 
    $username: string, 
    $password: string, 
    $displayname: option<string>,
    $accesstoken: option<string>,
){
    LET $entity_id = rand::uuid::v4();

    IF $displayname = NONE {
        LET $displayname = $username;
    };

    CREATE Entity CONTENT {
        id: $entity_id,
        deleted: false,
        displayname: $displayname,
        joined: time::now(),
        username: $username,
    };

    LET $human_id = rand::uuid::v4();

    CREATE Human CONTENT {
        id: $human_id,
        email: $email,
        passhash: crypto::argon2::generate($password),
        entity: Entity:entity_id
    };

    IF !$accesstoken = NONE {
        RELATE Human:human_id -> UserJoinedBy -> AccessToken:(SELECT id FROM AccessToken WHERE token = $accesstoken)[0]
    }

    RETURN $human_id
}

DEFINE FUNCTION objects::human::get($id: uuid) {
    
    LET $human = (SELECT * FROM Human WHERE id = $id)[0];
    LET $entity = (SELECT * FROM Entity WHERE id = $human.entity)[0];

    RETURN {
        id: $human.id,
        email: $human.email,
        passhash: $human.passhash,
        username: $entity.username,
        displayname: $entity.displayname,
        joined: $entity.joined
    }
}
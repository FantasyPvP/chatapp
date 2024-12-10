DEFINE FUNCTION auth::credentials::validate($username: string, $password: string) {
    LET $user = (SELECT id, passhash FROM User WHERE username = $username)[0];
    IF $user
        {
            RETURN {
                authenticated: crypto::argon2::compare($user.passhash, $password),
                id: $user.id
            };
        }
    ELSE
        { NONE }
    ;
}

DEFINE FUNCTION auth::sessiontoken::new($user: uuid) {
    LET $token = rand::string(64);

    CREATE SessionToken CONTENT {
        id: rand::uuid::v4(),
        created: time::now(),
        expires: time::now() + 7d,
        token: $token,
        user: Entity:user,
    };
}

DEFINE FUNCTION auth::sessiontoken::validate($token: string) {
    LET $exists = (SELECT * FROM SessionToken WHERE token = $token)[0];

    IF $exists = NONE {
        RETURN NONE;
    }

    IF time::now() > $exists.expires {
        DELETE SessionToken:exists.id;
        RETURN NONE;
    };

    RETURN $exists.user;
}

DEFINE FUNCTION auth::accesstoken::new($user: uuid, $max_uses: option<int>) {

    IF $max_uses = NONE {
        LET $max_uses = 1;
    };

    LET $token = rand::string(64);

    CREATE AccessToken CONTENT {
        id: rand::uuid::v4(),
        max_uses: 1,
        token: $token,
        created: time::now(),
        expires: time::now() + 7d,
        creator: Entity:user
        uses: 0,
    };

    RETURN $token;
}


DEFINE FUNCTION auth::user::signup(
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

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


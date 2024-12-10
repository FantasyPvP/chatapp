-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- FUNCTIONS
-- ------------------------------

DEFINE FUNCTION fn::auth::credentials::validate($username: string, $password: string) -> option<object> {
    LET $user = (SELECT id, passhash FROM User WHERE username = $username)[0];
    IF $user { 
        RETURN { 
            authenticated: crypto::argon2::compare($user.passhash, $password), 
            id: $user.id 
        }; 
    } ELSE { 
        RETURN NONE 
    };
} PERMISSIONS FULL;


DEFINE FUNCTION fn::auth::sessiontoken::new($user: uuid) {
    LET $token = rand::string(64);
    CREATE SessionToken CONTENT { 
        created: time::now(), 
        expires: time::now() + 1w, 
        id: rand::uuid::v4(), 
        token: $token, 
        user: Entity:user 
    };
} PERMISSIONS FULL;


DEFINE FUNCTION fn::auth::sessiontoken::validate($token: string) {
    LET $exists = (SELECT * FROM SessionToken WHERE token = $token)[0];
    IF $exists = NONE { RETURN NONE; };
    IF time::now() > $exists.expires {
        DELETE SessionToken:exists.id;
        RETURN NONE;
    };
    RETURN $exists.user;
} PERMISSIONS FULL;


DEFINE FUNCTION fn::channel::new() {} PERMISSIONS FULL;


DEFINE FUNCTION fn::friend::accept($from: uuid, $to: uuid) {
    IF (SELECT * FROM HasFriendRequest WHERE in = Entity:from AND out = Entity:to)[0] = NONE { RETURN NONE; } ELSE { DELETE HasFriendRequest WHERE in = Entity:from AND out = Entity:to; };
    LET $fsid: uuid = rand::uuid::v4();
    CREATE Friendship SET id = $fsid, channel_id = fn::channel::new(), since = time::now();

    RELATE Entity:from -> HasFriendship -> Friendship:fsid CONTENT { 
        nickname = Entity:from.displayname 
    };

    RELATE Entity:to -> HasFriendship -> Friendship:fsid CONTENT { 
        nickname = Entity:to.displayname 
    };

} PERMISSIONS FULL;


DEFINE FUNCTION fn::friend::reject($from: uuid, $to: uuid) { 
    DELETE Entity:from->FriendRequest WHERE out = Entity:to; 
} PERMISSIONS FULL;


DEFINE FUNCTION fn::friend::remove($from: uuid, $to: uuid) { 
    DELETE Entity:from->HasFriendship->Friendship WHERE <-HasFriendship.out = Entity:to; 
} PERMISSIONS FULL;


DEFINE FUNCTION fn::friend::request($from: uuid, $to: uuid) { 
    RELATE Entity:from -> HasFriendRequest -> Entity:to CONTENT { 
        created = time::now() 
    }; 
} PERMISSIONS FULL;


DEFINE FUNCTION fn::icon::default() -> string { 
    RETURN '/static/public/server_default.png'; 
} PERMISSIONS FULL;


DEFINE FUNCTION fn::message::edit($user: uuid, $message: uuid, $content: string) {
    LET $id = (SELECT id FROM Entity:user->SentMessage->(Message WHERE id = $message)->HasContent->MessageStringContent)[0];
    UPDATE MessageStringContent SET content = $content WHERE id = $id.id;
    UPDATE Message:message SET edited = true;
} PERMISSIONS FULL;


DEFINE FUNCTION fn::server::join($entity: uuid, $server: uuid) {
    LET $user = (SELECT displayname FROM Entity WHERE id = $entity);
    RELATE Entity:entity -> HasServer -> Server:server CONTENT { 
        joined: time::now(), nickname: $user.displayname 
    };
} PERMISSIONS FULL;


DEFINE FUNCTION fn::server::leave($server: uuid, $entity: uuid) {
    DELETE HasServer WHERE out = Server:server AND in = Entity:entity;
    DELETE HasRole WHERE in = Entity:entity AND ->Role.server = Server:server;
} PERMISSIONS FULL;


DEFINE FUNCTION fn::server::new($name: string, $creator: uuid) {
    LET $id = rand::uuid::v4();
    CREATE Server CONTENT { 
        created: time::now(), 
        icon_uri: fn::icon::default(), 
        id: $id, 
        name: $name, 
        owner: $creator 
    };
    fn::server::join($id, $creator);
} PERMISSIONS FULL;

DEFINE FUNCTION fn::sessiontoken::new() -> string {
    LET $token = rand::string(64);
    CREATE SessionToken CONTENT { 
        created: time::now(), 
        expires: time::now() + 1w, 
        id: rand::uuid::v4(), 
        token: $token, 
        user: Entity:user 
    };
} PERMISSIONS FULL;

-- ------------------------------
-- TABLE: AccessToken
-- ------------------------------

DEFINE TABLE AccessToken TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD created ON AccessToken TYPE datetime PERMISSIONS FULL;
DEFINE FIELD creator ON AccessToken TYPE record<Human> PERMISSIONS FULL;
DEFINE FIELD expires ON AccessToken TYPE datetime PERMISSIONS FULL;
DEFINE FIELD id ON AccessToken TYPE uuid PERMISSIONS FULL;
DEFINE FIELD max_uses ON AccessToken TYPE option<int> PERMISSIONS FULL;
DEFINE FIELD token ON AccessToken TYPE string PERMISSIONS FULL;
DEFINE FIELD uses ON AccessToken TYPE int PERMISSIONS FULL;



-- ------------------------------
-- TABLE: Channel
-- ------------------------------

DEFINE TABLE Channel TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD id ON Channel TYPE uuid PERMISSIONS FULL;



-- ------------------------------
-- TABLE: DataPool
-- ------------------------------

DEFINE TABLE DataPool TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD `value` ON DataPool TYPE string PERMISSIONS FULL;
DEFINE FIELD id ON DataPool TYPE uuid PERMISSIONS FULL;
DEFINE FIELD key ON DataPool TYPE string PERMISSIONS FULL;
DEFINE FIELD owner ON DataPool TYPE record<Entity> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: Entity
-- ------------------------------

DEFINE TABLE Entity TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD deleted ON Entity TYPE bool VALUE false PERMISSIONS FULL;
DEFINE FIELD displayname ON Entity TYPE string PERMISSIONS FULL;
DEFINE FIELD id ON Entity TYPE uuid PERMISSIONS FULL;
DEFINE FIELD joined ON Entity TYPE datetime PERMISSIONS FULL;
DEFINE FIELD username ON Entity TYPE string PERMISSIONS FULL;



-- ------------------------------
-- TABLE: Faction
-- ------------------------------

DEFINE TABLE Faction TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD id ON Faction TYPE uuid PERMISSIONS FULL;
DEFINE FIELD name ON Faction TYPE string PERMISSIONS FULL;
DEFINE FIELD owner ON Faction TYPE record<Entity> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: Friendship
-- ------------------------------

DEFINE TABLE Friendship TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD dm_channel ON Friendship TYPE record<Channel> PERMISSIONS FULL;
DEFINE FIELD id ON Friendship TYPE uuid PERMISSIONS FULL;
DEFINE FIELD since ON Friendship TYPE datetime PERMISSIONS FULL;



-- ------------------------------
-- TABLE: GroupChat
-- ------------------------------

DEFINE TABLE GroupChat TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD channel ON GroupChat TYPE record<Channel> PERMISSIONS FULL;
DEFINE FIELD icon_uri ON GroupChat TYPE string PERMISSIONS FULL;
DEFINE FIELD name ON GroupChat TYPE string PERMISSIONS FULL;
DEFINE FIELD owner ON GroupChat TYPE record<Entity> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: HasChannel
-- ------------------------------

DEFINE TABLE HasChannel TYPE RELATION IN Server OUT Channel SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON HasChannel TYPE record<Server> PERMISSIONS FULL;
DEFINE FIELD out ON HasChannel TYPE record<Channel> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: HasContent
-- ------------------------------

DEFINE TABLE HasContent TYPE RELATION IN Message OUT MessageStringContent SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON HasContent TYPE record<Message> PERMISSIONS FULL;
DEFINE FIELD out ON HasContent TYPE record<MessageStringContent> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: HasDomain
-- ------------------------------

DEFINE TABLE HasDomain TYPE RELATION IN DataPool OUT Server | GroupChat | Friendship SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON HasDomain TYPE record<DataPool> PERMISSIONS FULL;
DEFINE FIELD out ON HasDomain TYPE record<Server | GroupChat | Friendship> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: HasFriendRequest
-- ------------------------------

DEFINE TABLE HasFriendRequest TYPE RELATION IN Entity OUT Entity SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD created ON HasFriendRequest TYPE datetime PERMISSIONS FULL;
DEFINE FIELD in ON HasFriendRequest TYPE record<Entity> PERMISSIONS FULL;
DEFINE FIELD out ON HasFriendRequest TYPE record<Entity> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: HasFriendship
-- ------------------------------

DEFINE TABLE HasFriendship TYPE RELATION IN Entity OUT Friendship SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON HasFriendship TYPE record<Entity> PERMISSIONS FULL;
DEFINE FIELD nickname ON HasFriendship TYPE string PERMISSIONS FULL;
DEFINE FIELD out ON HasFriendship TYPE record<Friendship> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: HasGroupChat
-- ------------------------------

DEFINE TABLE HasGroupChat TYPE RELATION IN Entity OUT GroupChat SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON HasGroupChat TYPE record<Entity> PERMISSIONS FULL;
DEFINE FIELD out ON HasGroupChat TYPE record<GroupChat> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: HasPermission
-- ------------------------------

DEFINE TABLE HasPermission TYPE RELATION IN HasServer | ServerRole OUT Permission SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON HasPermission TYPE record<HasServer | ServerRole> PERMISSIONS FULL;
DEFINE FIELD out ON HasPermission TYPE record<Permission> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: HasRole
-- ------------------------------

DEFINE TABLE HasRole TYPE RELATION IN Entity OUT ServerRole SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON HasRole TYPE record<Entity> PERMISSIONS FULL;
DEFINE FIELD out ON HasRole TYPE record<ServerRole> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: HasServer
-- ------------------------------

DEFINE TABLE HasServer TYPE RELATION IN Entity OUT Server SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON HasServer TYPE record<Entity> PERMISSIONS FULL;
DEFINE FIELD joined ON HasServer TYPE datetime PERMISSIONS FULL;
DEFINE FIELD nickname ON HasServer TYPE string PERMISSIONS FULL;
DEFINE FIELD out ON HasServer TYPE record<Server> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: HasTitle
-- ------------------------------

DEFINE TABLE HasTitle TYPE RELATION IN Entity OUT Title SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON HasTitle TYPE record<Entity> PERMISSIONS FULL;
DEFINE FIELD out ON HasTitle TYPE record<Title> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: Human
-- ------------------------------

DEFINE TABLE Human TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD email ON Human TYPE string PERMISSIONS FULL;
DEFINE FIELD entity ON Human TYPE record<Entity> PERMISSIONS FULL;
DEFINE FIELD id ON Human TYPE uuid PERMISSIONS FULL;
DEFINE FIELD passhash ON Human TYPE string PERMISSIONS FULL;



-- ------------------------------
-- TABLE: Message
-- ------------------------------

DEFINE TABLE Message TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD channel ON Message TYPE record<Channel> PERMISSIONS FULL;
DEFINE FIELD edited ON Message TYPE bool PERMISSIONS FULL;
DEFINE FIELD id ON Message TYPE uuid PERMISSIONS FULL;
DEFINE FIELD reply_to ON Message TYPE option<record<Message>> PERMISSIONS FULL;
DEFINE FIELD sent ON Message TYPE datetime PERMISSIONS FULL;



-- ------------------------------
-- TABLE: MessageStringContent
-- ------------------------------

DEFINE TABLE MessageStringContent TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD content ON MessageStringContent TYPE string PERMISSIONS FULL;
DEFINE FIELD id ON MessageStringContent TYPE uuid PERMISSIONS FULL;



-- ------------------------------
-- TABLE: Permission
-- ------------------------------

DEFINE TABLE Permission TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD id ON Permission TYPE uuid PERMISSIONS FULL;
DEFINE FIELD name ON Permission TYPE string PERMISSIONS FULL;
DEFINE FIELD server ON Permission TYPE record<Server> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: Plugin
-- ------------------------------

DEFINE TABLE Plugin TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD created ON Plugin TYPE datetime PERMISSIONS FULL;
DEFINE FIELD id ON Plugin TYPE uuid PERMISSIONS FULL;
DEFINE FIELD name ON Plugin TYPE string PERMISSIONS FULL;



-- ------------------------------
-- TABLE: PluginEntity
-- ------------------------------

DEFINE TABLE PluginEntity TYPE RELATION IN Plugin OUT Entity SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON PluginEntity TYPE record<Plugin> PERMISSIONS FULL;
DEFINE FIELD out ON PluginEntity TYPE record<Entity> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: SentMessage
-- ------------------------------

DEFINE TABLE SentMessage TYPE RELATION IN Entity OUT Message SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON SentMessage TYPE record<Entity> PERMISSIONS FULL;
DEFINE FIELD out ON SentMessage TYPE record<Message> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: Server
-- ------------------------------

DEFINE TABLE Server TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD created ON Server TYPE datetime PERMISSIONS FULL;
DEFINE FIELD icon_uri ON Server TYPE string PERMISSIONS FULL;
DEFINE FIELD id ON Server TYPE uuid PERMISSIONS FULL;
DEFINE FIELD name ON Server TYPE string PERMISSIONS FULL;
DEFINE FIELD owner ON Server TYPE record<Entity> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: ServerRole
-- ------------------------------

DEFINE TABLE ServerRole TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD colour ON ServerRole TYPE string PERMISSIONS FULL;
DEFINE FIELD name ON ServerRole TYPE string PERMISSIONS FULL;
DEFINE FIELD server ON ServerRole TYPE record<Server> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: SessionToken
-- ------------------------------

DEFINE TABLE SessionToken TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD created ON SessionToken TYPE datetime PERMISSIONS FULL;
DEFINE FIELD expires ON SessionToken TYPE datetime PERMISSIONS FULL;
DEFINE FIELD id ON SessionToken TYPE uuid PERMISSIONS FULL;
DEFINE FIELD token ON SessionToken TYPE string PERMISSIONS FULL;
DEFINE FIELD user ON SessionToken TYPE record<Human> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: Title
-- ------------------------------

DEFINE TABLE Title TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD colour ON Title TYPE string PERMISSIONS FULL;
DEFINE FIELD faction ON Title TYPE record<Faction> PERMISSIONS FULL;
DEFINE FIELD name ON Title TYPE string PERMISSIONS FULL;



-- ------------------------------
-- TABLE: UserJoinedBy
-- ------------------------------

DEFINE TABLE UserJoinedBy TYPE RELATION IN Human OUT AccessToken SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON UserJoinedBy TYPE record<Human> PERMISSIONS FULL;
DEFINE FIELD out ON UserJoinedBy TYPE record<AccessToken> PERMISSIONS FULL;




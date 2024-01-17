enum Short {
    UPPER=0xFF
    LOWER=0x00,
    RESET=UPPER,
    STRAP=LOWER
}

union Ptr {
    union UTF8 {
        unsigned char,
        char,
        Short
    };

    union UTF16 {
        unsigned short,
        short,
        UTF8
    };

    union Full {
        unsigned long,
        long,
        Half
    };

    union Long {
        void*,
        Long, 
        Data::Full
    };
};

union Data {
    union Str {
        const char*
    };

    union Unicode {
        Ptr::Unicode
    };

    union Half {
        Ptr::Half
    };

    union Full {
        Ptr::Full
    };

    union Long {
        unsigned long long,
        long long,
        unsigned long int,
        long int,
        int,
        Ptr::Long
    };
};

struct NamedKV {
    static char* name;
    unsigned long id;
};

union KVPair {
    union key {
        Data::Half,
        NamedKV
    };

    union value {
        Data::Half
    };

    struct kv_store {
        KVPair::key,
        KVPair::value
    };
};

static unsigned long TICKER = 0;

Ptr make_kv_pair_key(name: Data::Str) {
    NamedKV key;
    key.name = key;
    Ptr::Full full;
    full.
    key.id = ++TICKER;
    return key;
}

Data make_kv_pair_value(value: Ptr::Long) {
    static const Data::Str NAME = "ehptloader";
    NamedKV value;
    value.name = NAME;
    value.id = (unsigned long) value;
    return value;
}

KVPair make_kv_pair(key: Data::Str, value: Ptr::Long) {
    KVPair thin;
    thin.key = make_kv_pair_key(key);
    thin.value = make_kv_pair_value(value);
    return thin.kv_pair;
}

Data get(Key key) {
    Key key = make_kv_pair_key(key);
    return (Data) key; // TODO: Add storage
}

Key set(key: Data::Str, value: Ptr::Long) {
    KVPair pair = make_kv_pair(key, value);
    return pair;
}
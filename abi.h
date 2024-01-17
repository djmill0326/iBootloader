enum Short {
    UPPER=0xFF,
    LOWER=0x00,
    RESET=UPPER,
    STRAP=LOWER
};

union Ptr {
    union UTF8 {
        unsigned char UChar;
        char Char;
        Short Unicode;
    };

    union UTF16 {
        unsigned short UShort;
        short Short;
        UTF8 Unicode;
    };

    union Full {
        unsigned long ULong;
        long Long;
        UTF16 Unicode;
    };

    union Long {
        void* c_void;
        Long* ptr;
        Data::Full Long;
    };

    NamedKV kv;
};

union Data {
    union Str {
        const char* String;
    };

    union Unicode {
        Ptr::UTF8 UTF8;
        Ptr::UTF16 UTF16;
        Ptr::Full Unicode;
    };

    union Half {
        Ptr::Full Full;
    };

    union Full {
        Ptr::Long Long;
    };

    union Long {
        unsigned long long ULL;
        long long LL;
        unsigned long int UInt;
        long int Int;
        int IInt;
        Ptr::Long Full;
    };
};

struct NamedKV {
    Data::Str name;
    Data::Half id;
};

union KVPair {
    union key {
        Data::Half data;
        NamedKV named_kv;
    };

    union value {
        Data::Half data;
    };

    struct kv_store {
        KVPair::key k;
        KVPair::value v;
    };
};

static unsigned long TICKER = 0;

Ptr make_kv_pair_key(Data::Str name) {
    NamedKV kv;
    kv.name = name;
    kv.id = ++TICKER;
    Ptr ptr;
    ptr.kv = kv;
}

Data make_kv_pair_value(Data value) {
    NamedKV kv;
    kv.name = *&"hapt-store";
    kv.id = value;
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
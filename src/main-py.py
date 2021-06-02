#[print(k) for k in ["["]+list(["   Arg {0} : \"{1}\"".format(x+1,list([__import__("sys").argv[x] for x in range(len(__import__("sys").argv))])[x+1]) for x in range(len(list([__import__("sys").argv[x] for x in range(len(__import__("sys").argv))]))-1)])+["]"]]

[ \
    print(k) for k in ["["] + list( \
        ["   Arg {0} : \"{1}\"".format( \
        x + 1,
        list( \
            [ \
                __import__("sys").argv[x] for x in range( \
                len(__import__("sys").argv)) \
            ] \
        )[x + 1]) for x in range( \
            len( \
                list( \
                    [ \
                        __import__("sys").argv[x] for x in range( \
                        len(__import__("sys").argv)) \
                    ] \
                ) \
            )-1 \
        ) \
    ]) + ["]"] \
]

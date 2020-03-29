// built-in include guard removal
// just in case the user has a local dependency with the same file name
#if defined _inc_yaml
	#undef _inc_yaml
#endif
// custom include-guard to ensure we don't duplicate
#if defined _yaml_included
	#endinput
#endif
#define _yaml_included

enum YAML_NODE {
    YAML_NODE_NUMBER,
    YAML_NODE_BOOLEAN,
    YAML_NODE_STRING,
    YAML_NODE_OBJECT,
    YAML_NODE_ARRAY,
    YAML_NODE_NULL,
}

// YAML_Parse decodes YAML and stores the root node into `output`.
native YAML_Parse(const string[], &Node:output);

// YAML_Stringify encodes a YAML node into `buf`.
native YAML_Stringify(Node:node, buf[], len = sizeof(buf));

// YAML_NodeType returns the type of a node from the above enumerator.
native YAML_NODE:YAML_NodeType(Node:node);

// YAML_Object allocates a node from a set of key-value pairs where each key must
// be a string and each value must be a `Node:` value. For example:
//
// YAML_Object("key", YAML_String("value"));
//
// output: {"key": "value"}
//
// Returns a `Node:` ID which can be passed as an argument to another YAML_Object
// function in order to build nested objects. For example:
//
// YAML_Object("key", YAML_Object("nestedKey", YAML_String("value")));
//
// output: {"key": {"nestedKey": "value"}}
//
native Node:YAML_Object({_, Node}:...);

// YAML_Int, YAML_Bool, YAML_Float, YAML_String each allocate a YAML node.
native Node:YAML_Int(value);
native Node:YAML_Bool(bool:value);
native Node:YAML_Float(Float:value);
native Node:YAML_String(const value[]);

// YAML_Array simply takes an argument list of `Node:` IDs.
//
// YAML_Array(YAML_String("value"), YAML_Int(1), YAML_Object("k", YAML_String("v")))
//
// output: ["value", 1, {"k": "v"}]
//
native Node:YAML_Array(Node:...);

// YAML_Append returns a new `Node:` which is the result of appending b to a.
// This works on both objects and arrays and the two input nodes will be deleted
// from the global node store. For example:
//
// new Node:a = YAML_Object("key1", YAML_String("value"));
// new Node:b = YAML_Object("key2", YAML_String("value"));
// new Node:c = YAML_Append(a, b);
//
// output: {"key1": "value", "key2": "value"}
//
// new Node:a = YAML_Array(YAML_Int(1), YAML_Int(2));
// new Node:a = YAML_Array(YAML_Int(3));
// new Node:c = YAML_Append(a, b);
//
// output: [1, 2, 3]
//
native Node:YAML_Append(Node:a, Node:b);
native Node:operator+(Node:a, Node:b) = YAML_Append;

// YAML_Set* functions directly modify nodes by inserting or modifying keys.
native YAML_SetObject(Node:node, const key[], Node:object);
native YAML_SetInt(Node:node, const key[], output);
native YAML_SetFloat(Node:node, const key[], Float:output);
native YAML_SetBool(Node:node, const key[], bool:output);
native YAML_SetString(Node:node, const key[], output[], len = sizeof(output));

// YAML_GetObject returns the `Node:` stored at `key` in the given `node`.
// For example:
//
// input: {"key": {"inner": 1}}
//
// new Node:output;
// YAML_GetObject(node, "key", output);
//
// `output` now contains a YAML object containing {"inner": 1}, this node can be
// treated like any other node:
//
// new outputValue;
// YAML_GetInt(output, outputValue);
// outputValue == 1
//
native YAML_GetObject(Node:node, const key[], &Node:output);

// YAML_Get* functions extract a native type from an object these functions are
// shorthand for:
//
// new Node:output;
// YAML_GetObject(node, "key", output);
// new string[128];
// YAML_GetNodeString(output, string);
//
// 99% of the time, you only need these functions to get values out of objects.
//
native YAML_GetInt(Node:node, const key[], &output);
native YAML_GetFloat(Node:node, const key[], &Float:output);
native YAML_GetBool(Node:node, const key[], &bool:output);
native YAML_GetString(Node:node, const key[], output[], len = sizeof(output));

// YAML_GetArray returns the `Node:` stored at `index` in the given `node`. The
// `Node:` returned could be an Object or a primitive, such as an int, float,
// bool or string. Use functions below to convert `Node:` into a native type.
// For example:
//
// input: {"key": [1, 2, 3]}
//
// new Node:output;
// YAML_GetArray(node, key, output);
//
// `output` now contains a YAML array and can be accessed with:
//
// new Node:element;
// YAML_ArrayObject(node, 1, element);
//
// `element` now contains a YAML integer type node and can be converted to a
// native integer type using `YAML_GetNodeInt`.
//
native YAML_GetArray(Node:node, const key[], &Node:output);
native YAML_ArrayLength(Node:node, &length);
native YAML_ArrayObject(Node:node, index, &Node:output);

// YAML_GetNode* functions extract a YAML object `Node:` to `output`.
// These are useful for when you get a `Node:` that represents a primitive type
// such as from YAML_GetArray.
native YAML_GetNodeInt(Node:node, &output);
native YAML_GetNodeFloat(Node:node, &Float:output);
native YAML_GetNodeBool(Node:node, &bool:output);
native YAML_GetNodeString(Node:node, output[], len = sizeof(output));

// YAML_ToggleGC toggles garbage collection for a node. This prevents
// `YAML_Cleanup` from deleting nodes if `auto` is true. In other words,
// disabling garbage collection for a node will prevent it from being deleted
// automatically when it leaves scope. This is useful for when you want to pass
// a node through function calls or store it for a longer period of time.
// Be very careful with this function as losing a node pointer will result in a
// classic memory leak. For example:
//
// new Node:n = YAML_Object();
// YAML_ToggleGC(n, false);
// CallLocalFunction("FillYAML_Object", "d", _:n);
// YAML_ToggleGC(n, true);
//
// This will ensure that each hook of `FillYAML_Object` does not delete `n` when
// it leaves scope.
//
native YAML_ToggleGC(Node:node, bool:toggle);

// -
// Internal
// -

// YAML_Cleanup is an internal function for cleaning up `Node:` objects. This is
// necessary because each of the object builder functions above allocate nodes
// in a pool to be passed between internal function calls. If called manually,
// leave `auto` as the default value of false which will ignore a garbage
// collection disable done with `YAML_ToggleGC`.
native YAML_Cleanup(Node:node, auto = false);

// cleans up nodes once they go out of scope.
stock operator~(const Node:nodes[], len) {
    for(new i; i < len; ++i) {
        YAML_Cleanup(nodes[i], true);
    }
}

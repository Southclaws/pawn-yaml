#define RUN_TESTS

#include <a_samp>
#include <YSI_Core\y_testing>

#include "yaml.inc"

main() {
    SetTimer("timeout", 10000, false);
}

forward timeout();
public timeout() {
    SendRconCommand("exit");
}

Test:YAML_Parse() {
    new Node:node;
    new ret;

    new input[] = "{\"list\":[{\"a_listobj_float\":66.5999984741211,\"a_listobj_number\":76,\"a_listobj_string\":\"another value\",\"one\":\"value one\"},{\"a_listobj_float\":66.5999984741211,\"a_listobj_number\":76,\"a_listobj_string\":\"another value\",\"two\":\"value two\"},{\"a_listobj_float\":66.5999984741211,\"a_listobj_number\":76,\"a_listobj_string\":\"another value\",\"three\":\"value three\"}],\"object\":{\"a_float\":66.5999984741211,\"a_number\":76,\"a_string\":\"a value\",\"nested_object\":{\"a_deeper_float\":66.5999984741211,\"a_deeper_number\":76,\"a_deeper_string\":\"another value\"}}}";

    ret = YAML_Parse(input, node);
    ASSERT_EQ(ret, 0);

    new output[1024];
    ret = YAML_Stringify(node, output);
    ASSERT(!strcmp(input, output));
}

Test:YAML_NodeType() {
    new Node:number = YAML_Int(3); // YAML_NODE_NUMBER
    ASSERT(YAML_NodeType(number) ==  YAML_NODE_NUMBER);

    new Node:boolean = YAML_Bool(true); // YAML_NODE_BOOLEAN
    ASSERT(YAML_NodeType(boolean) ==  YAML_NODE_BOOLEAN);

    new Node:string = YAML_String("hi"); // YAML_NODE_STRING
    ASSERT(YAML_NodeType(string) ==  YAML_NODE_STRING);

    new Node:object = YAML_Object("k", YAML_Int(1)); // YAML_NODE_OBJECT
    ASSERT(YAML_NodeType(object) ==  YAML_NODE_OBJECT);

    new Node:array = YAML_Array(YAML_Int(1), YAML_Int(2)); // YAML_NODE_ARRAY
    ASSERT(YAML_NodeType(array) ==  YAML_NODE_ARRAY);

    new Node:null = Node:-1; // YAML_NODE_NULL
    ASSERT(YAML_NodeType(null) ==  YAML_NODE_NULL);
}

Test:YAML_ObjectEmpty() {
    new Node:node = YAML_Object();

    new buf[128];
    new ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "{}"));
}

Test:YAML_ObjectInt() {
    new Node:node = YAML_Object(
        "key", YAML_Int(1)
    );

    new buf[128];
    new ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "{\"key\":1}"));
    print(buf);
}

Test:YAML_ObjectInts() {
    new Node:node = YAML_Object(
        "key1", YAML_Int(1),
        "key2", YAML_Int(2),
        "key3", YAML_Int(3)
    );

    new buf[128];
    new ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "{\"key1\":1,\"key2\":2,\"key3\":3}"));
    print(buf);
}

Test:YAML_ObjectFloat() {
    new Node:node = YAML_Object(
        "key", YAML_Float(1.5)
    );

    new buf[128];
    new ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "{\"key\":1.5}"));
    print(buf);
}

Test:YAML_ObjectFloats() {
    new Node:node = YAML_Object(
        "key1", YAML_Float(1.5),
        "key2", YAML_Float(2.5),
        "key3", YAML_Float(3.5)
    );

    new buf[128];
    new ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "{\"key1\":1.5,\"key2\":2.5,\"key3\":3.5}"));
    print(buf);
}

Test:YAML_ObjectBool() {
    new Node:node = YAML_Object(
        "key", YAML_Bool(true)
    );

    new buf[128];
    new ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "{\"key\":true}"));
    print(buf);
}

Test:YAML_ObjectBools() {
    new Node:node = YAML_Object(
        "key1", YAML_Bool(false),
        "key2", YAML_Bool(true),
        "key3", YAML_Bool(false)
    );

    new buf[128];
    new ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "{\"key1\":false,\"key2\":true,\"key3\":false}"));
    print(buf);
}

Test:YAML_ObjectString() {
    new Node:node = YAML_Object(
        "key", YAML_String("value")
    );

    new buf[128];
    new ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "{\"key\":\"value\"}"));
    print(buf);
}

Test:YAML_ObjectStrings() {
    new Node:node = YAML_Object(
        "key1", YAML_String("value1"),
        "key2", YAML_String("value2"),
        "key3", YAML_String("value3")
    );

    new buf[128];
    new ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "{\"key1\":\"value1\",\"key2\":\"value2\",\"key3\":\"value3\"}"));
    print(buf);
}

Test:YAML_StringArray() {
    new Node:node = YAML_Array(
        YAML_String("one"),
        YAML_String("two"),
        YAML_String("three")
    );

    new buf[128];
    new ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "[\"one\",\"two\",\"three\"]"));
    print(buf);
}

Test:YAML_IntArray() {
    new Node:node = YAML_Array(
        YAML_Int(1),
        YAML_Int(2),
        YAML_Int(3)
    );

    new buf[128];
    new ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "[1,2,3]"));
    print(buf);
}

Test:YAML_FloatArray() {
    new Node:node = YAML_Array(
        YAML_Float(1.5),
        YAML_Float(2.5),
        YAML_Float(3.5)
    );

    new buf[128];
    new ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "[1.5,2.5,3.5]"));
    print(buf);
}

Test:YAML_ObjectArray() {
    new Node:node = YAML_Array(
        YAML_Object(
            "one", YAML_String("value one")
        ),
        YAML_Object(
            "two", YAML_String("value two")
        ),
        YAML_Object(
            "three", YAML_String("value three")
        )
    );

    new buf[128];
    new ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "[{\"one\":\"value one\"},{\"two\":\"value two\"},{\"three\":\"value three\"}]"));
    print(buf);
}

/*
YAML_ObjectComplex generates this rather complex YAML object:
{
  "object": {
    "a_float": 66.599998474121094,
    "a_number": 76,
    "a_string": "a value",
    "nested_object": {
      "a_deeper_float": 66.599998474121094,
      "a_deeper_number": 76,
      "a_deeper_string": "another value"
    }
  },
  "list": [
    {
      "a_listobj_float": 66.599998474121094,
      "a_listobj_number": 76,
      "a_listobj_string": "another value",
      "one": "value one"
    },
    {
      "a_listobj_float": 66.599998474121094,
      "a_listobj_number": 76,
      "a_listobj_string": "another value",
      "two": "value two"
    },
    {
      "a_listobj_float": 66.599998474121094,
      "a_listobj_number": 76,
      "a_listobj_string": "another value",
      "three": "value three"
    }
  ]
}
*/
Test:YAML_ObjectComplex() {
    new Node:node = YAML_Object(
        "object", YAML_Object(
            "a_string", YAML_String("a value"),
            "a_number", YAML_Int(76),
            "a_float", YAML_Float(66.6),
            "nested_object", YAML_Object(
                "a_deeper_string", YAML_String("another value"),
                "a_deeper_number", YAML_Int(76),
                "a_deeper_float", YAML_Float(66.6)
            )
        ),
        "list", YAML_Array(
            YAML_Object(
                "one", YAML_String("value one"),
                "a_listobj_string", YAML_String("another value"),
                "a_listobj_number", YAML_Int(76),
                "a_listobj_float", YAML_Float(66.6)
            ),
            YAML_Object(
                "two", YAML_String("value two"),
                "a_listobj_string", YAML_String("another value"),
                "a_listobj_number", YAML_Int(76),
                "a_listobj_float", YAML_Float(66.6)
            ),
            YAML_Object(
                "three", YAML_String("value three"),
                "a_listobj_string", YAML_String("another value"),
                "a_listobj_number", YAML_Int(76),
                "a_listobj_float", YAML_Float(66.6)
            )
        )
    );

    new buf[1024];
    new ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "{\"list\":[{\"a_listobj_float\":66.5999984741211,\"a_listobj_number\":76,\"a_listobj_string\":\"another value\",\"one\":\"value one\"},{\"a_listobj_float\":66.5999984741211,\"a_listobj_number\":76,\"a_listobj_string\":\"another value\",\"two\":\"value two\"},{\"a_listobj_float\":66.5999984741211,\"a_listobj_number\":76,\"a_listobj_string\":\"another value\",\"three\":\"value three\"}],\"object\":{\"a_float\":66.5999984741211,\"a_number\":76,\"a_string\":\"a value\",\"nested_object\":{\"a_deeper_float\":66.5999984741211,\"a_deeper_number\":76,\"a_deeper_string\":\"another value\"}}}"));
    print(buf);
}

Test:YAML_AppendObject() {
    new Node:a = YAML_Object(
        "key1", YAML_String("value1"),
        "key2", YAML_String("value2")
    );
    new Node:b = YAML_Object(
        "key3", YAML_String("value3")
    );

    new Node:c = YAML_Append(a, b);

    new buf[128];
    new ret = YAML_Stringify(c, buf);
    ASSERT_EQ(ret, 0);
    ASSERT_SAME(buf, "{\"key1\":\"value1\",\"key2\":\"value2\",\"key3\":\"value3\"}");
    print(buf);
}

Test:YAML_AppendArray() {
    new Node:a = YAML_Array(
        YAML_Int(1),
        YAML_Int(2)
    );
    new Node:b = YAML_Array(
        YAML_Int(3)
    );

    new Node:c = YAML_Append(a, b);

    new buf[128];
    new ret = YAML_Stringify(c, buf);
    ASSERT_EQ(ret, 0);
    ASSERT_SAME(buf, "[1,2,3]");
    print(buf);
}

Test:YAML_SetObject() {
    new Node:node = YAML_Object();
    new ret = YAML_SetObject(node, "key", YAML_Object("key", YAML_String("value")));
    ASSERT(ret == 0);

    new buf[128];
    ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "{\"key\":{\"key\":\"value\"}}"));
    print(buf);
}

Test:YAML_SetInt() {
    new Node:node = YAML_Object();
    new ret = YAML_SetInt(node, "key", 5);
    ASSERT(ret == 0);

    new buf[128];
    ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "{\"key\":5}"));
    print(buf);
}

Test:YAML_SetFloat() {
    new Node:node = YAML_Object();
    new ret = YAML_SetFloat(node, "key", 5.5);
    ASSERT(ret == 0);

    new buf[128];
    ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "{\"key\":5.5}"));
    print(buf);
}

Test:YAML_SetBool() {
    new Node:node = YAML_Object();
    new ret = YAML_SetBool(node, "key", true);
    ASSERT(ret == 0);

    new buf[128];
    ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "{\"key\":true}"));
    print(buf);
}

Test:YAML_SetString() {
    new Node:node = YAML_Object();
    new ret = YAML_SetString(node, "key", "value");
    ASSERT(ret == 0);

    new buf[128];
    ret = YAML_Stringify(node, buf);
    ASSERT(ret == 0);
    ASSERT(!strcmp(buf, "{\"key\":\"value\"}"));
    print(buf);
}

Test:YAML_GetInt() {
    new Node:node = YAML_Object(
        "key1", YAML_Int(1),
        "key2", YAML_Int(2),
        "key3", YAML_Int(3)
    );

    new got;
    new ret;
    
    ret = YAML_GetInt(node, "key1", got);
    ASSERT_EQ(ret, 0);
    ASSERT_EQ(got, 1);

    ret = YAML_GetInt(node, "key2", got);
    ASSERT_EQ(ret, 0);
    ASSERT_EQ(got, 2);

    ret = YAML_GetInt(node, "key3", got);
    ASSERT_EQ(ret, 0);
    ASSERT_EQ(got, 3);

    ret = YAML_GetInt(node, "key4", got);
    ASSERT_EQ(ret, 2);
}

Test:YAML_GetFloat() {
    new Node:node = YAML_Object(
        "key1", YAML_Float(1.5),
        "key2", YAML_Float(2.5),
        "key3", YAML_Float(3.5)
    );

    new Float:got;
    new ret;
    
    ret = YAML_GetFloat(node, "key1", got);
    ASSERT(ret == 0);
    ASSERT(got == 1.5);

    ret = YAML_GetFloat(node, "key2", got);
    ASSERT(ret == 0);
    ASSERT(got == 2.5);

    ret = YAML_GetFloat(node, "key3", got);
    ASSERT(ret == 0);
    ASSERT(got == 3.5);

    ret = YAML_GetFloat(node, "key4", got);
    ASSERT(ret == 2);
}

Test:YAML_GetBool() {
    new Node:node = YAML_Object(
        "key1", YAML_Bool(false),
        "key2", YAML_Bool(true),
        "key3", YAML_Bool(false)
    );

    new bool:got;
    new ret;
    
    ret = YAML_GetBool(node, "key1", got);
    ASSERT(ret == 0);
    ASSERT(got == false);

    ret = YAML_GetBool(node, "key2", got);
    ASSERT(ret == 0);
    ASSERT(got == true);

    ret = YAML_GetBool(node, "key3", got);
    ASSERT(ret == 0);
    ASSERT(got == false);

    ret = YAML_GetBool(node, "key4", got);
    ASSERT(ret == 2);
}

Test:YAML_GetString() {
    new Node:node = YAML_Object(
        "key1", YAML_String("value1"),
        "key2", YAML_String("value2"),
        "key3", YAML_String("value3")
    );

    new got[128];
    new ret;
    
    ret = YAML_GetString(node, "key1", got);
    ASSERT(ret == 0);
    ASSERT(!strcmp(got, "value1"));

    ret = YAML_GetString(node, "key2", got);
    ASSERT(ret == 0);
    ASSERT(!strcmp(got, "value2"));

    ret = YAML_GetString(node, "key3", got);
    ASSERT(ret == 0);
    ASSERT(!strcmp(got, "value3"));

    ret = YAML_GetString(node, "key4", got);
    ASSERT(ret == 2);
}

Test:YAML_GetArray() {
    new Node:node = YAML_Object(
        "key1", YAML_Array(
            YAML_String("one"),
            YAML_String("two"),
            YAML_String("three")
        )
    );

    new Node:arrayNode;
    new ret;

    ret = YAML_GetArray(node, "key1", arrayNode);
    printf("YAML_GetArray:%d arrayNode: %d", ret, _:arrayNode);
    ASSERT_EQ(ret, 0);

    new Node:output;
    new gotString[32];

    ret = YAML_ArrayObject(arrayNode, 0, output);
    ASSERT_EQ(ret, 0);
    ret = YAML_GetNodeString(output, gotString);
    ASSERT_EQ(ret, 0);
    ASSERT_SAME(gotString, "one");

    ret = YAML_ArrayObject(arrayNode, 1, output);
    ASSERT_EQ(ret, 0);
    ret = YAML_GetNodeString(output, gotString);
    ASSERT_EQ(ret, 0);
    ASSERT_SAME(gotString, "two");

    ret = YAML_ArrayObject(arrayNode, 2, output);
    ASSERT_EQ(ret, 0);
    ret = YAML_GetNodeString(output, gotString);
    ASSERT_EQ(ret, 0);
    ASSERT_SAME(gotString, "three");
}

Test:YAML_GetIntInvalid() {
    new Node:node = YAML_Object("k", YAML_String("v"));
    new gotInt;
    new ret = YAML_GetInt(node, "key4", gotInt);
    ASSERT(ret == 2);
}

Test:YAML_GetFloatInvalid() {
    new Node:node = YAML_Object("k", YAML_String("v"));
    new Float:gotFloat;
    new ret = YAML_GetFloat(node, "key4", gotFloat);
    ASSERT(ret == 2);
}

Test:YAML_GetBoolInvalid() {
    new Node:node = YAML_Object("k", YAML_String("v"));
    new bool:gotBool;
    new ret = YAML_GetBool(node, "key4", gotBool);
    ASSERT(ret == 2);
}

Test:YAML_GetStringInvalid() {
    new Node:node = YAML_Object("k", YAML_String("v"));
    new gotString[1];
    new ret = YAML_GetString(node, "key4", gotString);
    ASSERT(ret == 2);
}

Test:YAML_GetArrayInvalid() {
    new Node:node = YAML_Object("k", YAML_String("v"));
    new Node:gotNode;
    new ret = YAML_GetArray(node, "key4", gotNode);
    ASSERT(ret == 2);
}

Test:YAML_ArrayLength() {
    new Node:node = YAML_Array(
        YAML_String("one"),
        YAML_String("two"),
        YAML_String("three")
    );

    new length;
    new ret;
    ret = YAML_ArrayLength(node, length);
    printf("ret %d length %d", ret, length);
    ASSERT(ret == 0);
    ASSERT(length == 3);
}

Test:YAML_ArrayObject() {
    new Node:node = YAML_Array(
        YAML_String("one"),
        YAML_String("two"),
        YAML_String("three")
    );

    new Node:output;
    new ret;
    ret = YAML_ArrayObject(node, 1, output);
    ASSERT(ret == 0);

    new got[32];
    ret = YAML_GetNodeString(output, got);
    ASSERT(ret == 0);
    ASSERT(!strcmp(got, "two"));
}

Test:YAML_GetNodeInt() {
    new Node:node = YAML_Object(
        "key", YAML_Int(1)
    );

    new Node:output;
    new ret;
    ret = YAML_GetObject(node, "key", output);
    ASSERT_EQ(ret, 0);

    new got;
    ret = YAML_GetNodeInt(output, got);
    ASSERT_EQ(ret, 0);
    ASSERT_EQ(got, 1);
}

Test:YAML_GetNodeFloat() {
    new Node:node = YAML_Object(
        "key", YAML_Float(1.34)
    );

    new Node:output;
    new ret;
    ret = YAML_GetObject(node, "key", output);
    ASSERT(ret == 0);

    new Float:got;
    ret = YAML_GetNodeFloat(output, got);
    ASSERT(ret == 0);
    ASSERT(got == 1.34);
}

Test:YAML_GetNodeBool() {
    new Node:node = YAML_Object(
        "key", YAML_Bool(true)
    );

    new Node:output;
    new ret;
    ret = YAML_GetObject(node, "key", output);
    ASSERT(ret == 0);

    new bool:got;
    ret = YAML_GetNodeBool(output, got);
    ASSERT(ret == 0);
    ASSERT(got == true);
}

Test:YAML_GetNodeString() {
    new Node:node = YAML_Object(
        "key", YAML_String("value")
    );

    new Node:output;
    new ret;
    ret = YAML_GetObject(node, "key", output);
    ASSERT(ret == 0);

    new got[32];
    ret = YAML_GetNodeString(output, got);
    ASSERT(ret == 0);
    ASSERT(!strcmp(got, "value"));
}

Test:YAML_ScopeGC() {
    new Node:node = YAML_Object();
    scopeNodeGC(node);
    ASSERT(YAML_Cleanup(node) == 1);
}

Test:YAML_ToggleGC() {
    new Node:node = YAML_Object(
        "key", YAML_String("value")
    );
    YAML_ToggleGC(node, false);
    scopeNodeGC(node);
    new value[6];
    YAML_GetString(node, "key", value);
    ASSERT_SAME(value, "value");
    ASSERT_EQ(YAML_Cleanup(node), 0);
    ASSERT_EQ(YAML_Cleanup(node), 1);
}

scopeNodeGC(Node:node) {
    printf("scoped %d", _:node);
}

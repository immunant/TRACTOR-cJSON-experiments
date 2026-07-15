/*
  Copyright (c) 2009-2017 Dave Gamble and cJSON contributors

  Permission is hereby granted, free of charge, to any person obtaining a copy
  of this software and associated documentation files (the "Software"), to deal
  in the Software without restriction, including without limitation the rights
  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
  copies of the Software, and to permit persons to whom the Software is
  furnished to do so, subject to the following conditions:

  The above copyright notice and this permission notice shall be included in
  all copies or substantial portions of the Software.

  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
  THE SOFTWARE.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "unity/unity_config.h"
#include "unity/unity.h"
#include "common.h"

static size_t encode_utf8(unsigned int codepoint, unsigned char output[5])
{
    size_t length = 0;

    if (codepoint <= 0x7F)
    {
        output[length++] = (unsigned char)codepoint;
    }
    else if (codepoint <= 0x7FF)
    {
        output[length++] = (unsigned char)(0xC0 | (codepoint >> 6));
        output[length++] = (unsigned char)(0x80 | (codepoint & 0x3F));
    }
    else
    {
        output[length++] = (unsigned char)(0xE0 | (codepoint >> 12));
        output[length++] = (unsigned char)(0x80 | ((codepoint >> 6) & 0x3F));
        output[length++] = (unsigned char)(0x80 | (codepoint & 0x3F));
    }
    output[length] = '\0';
    return length;
}

static void parse_hex4_should_parse_all_observable_bmp_combinations(void)
{
    unsigned int number = 0;
    char json[16];
    unsigned char expected[5];

    for (number = 0; number <= 0xFFFF; number++)
    {
        cJSON *item = NULL;
        size_t expected_length = 0;

        if ((number >= 0xD800) && (number <= 0xDFFF))
        {
            continue;
        }

        TEST_ASSERT_EQUAL_INT(8, snprintf(json, sizeof(json), "\"\\u%04x\"", number));
        expected_length = encode_utf8(number, expected);
        item = cJSON_ParseWithOpts(json, NULL, true);
        TEST_ASSERT_NOT_NULL_MESSAGE(item, "Failed to parse a BMP Unicode escape.");
        TEST_ASSERT_TRUE(cJSON_IsString(item));
        TEST_ASSERT_EQUAL_UINT8_ARRAY(expected, item->valuestring, expected_length + 1);
        cJSON_Delete(item);
    }
}

static void parse_hex4_should_parse_mixed_case(void)
{
    static const char *cases[] = {
        "beef", "beeF", "beEf", "beEF", "bEef", "bEeF", "bEEf", "bEEF",
        "Beef", "BeeF", "BeEf", "BeEF", "BEef", "BEeF", "BEEf", "BEEF"
    };
    const unsigned char expected[] = {0xEB, 0xBB, 0xAF, 0};
    size_t index = 0;

    for (index = 0; index < (sizeof(cases) / sizeof(cases[0])); index++)
    {
        char json[16];
        cJSON *item = NULL;
        TEST_ASSERT_EQUAL_INT(8, snprintf(json, sizeof(json), "\"\\u%s\"", cases[index]));
        item = cJSON_ParseWithOpts(json, NULL, true);
        TEST_ASSERT_NOT_NULL(item);
        TEST_ASSERT_EQUAL_UINT8_ARRAY(expected, item->valuestring, sizeof(expected));
        cJSON_Delete(item);
    }
}

int CJSON_CDECL main(void)
{
    UNITY_BEGIN();
    RUN_TEST(parse_hex4_should_parse_all_observable_bmp_combinations);
    RUN_TEST(parse_hex4_should_parse_mixed_case);
    return UNITY_END();
}

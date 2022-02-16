import test from 'ava';
import {parse, parseSync} from "../index";

let tomlString = ` 
    [test]
    foo = "bar"
`;

let result =
    {
        test: {
            foo: 'bar',
        }
    }


test('parse', async t => {
    t.is(JSON.stringify(await parse(tomlString)), JSON.stringify(result));
});

test('parseSync', async t => {
    t.is(JSON.stringify(parseSync(tomlString)), JSON.stringify(result));
});
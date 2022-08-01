#!/usr/bin/env python
import lexical_analyzer
import binary_tree
import tokens


def build_env_from_history(history):
    return {f"x{i}": value for i, value in enumerate(history)}


def preprocess_tokens(_tokens):
    i = 0
    while i < len(_tokens) - 1:
        if isinstance(_tokens[i], tokens.Minus) and (
            i == 0
            or (
                not isinstance(
                    _tokens[i - 1], (tokens.Variable, tokens.Number, tokens.CloseParen)
                )
            )
        ):
            if isinstance(_tokens[i + 1], (tokens.Variable, tokens.OpenParen)):
                _tokens.pop(i)
                _tokens.insert(i, tokens.Multiply())
                _tokens.insert(i, tokens.Integer("-1"))
            elif isinstance(_tokens[i + 1], tokens.Number):
                _tokens.pop(i)
                _tokens[i].value = "-" + _tokens[i].value
        elif isinstance(
            _tokens[i], (tokens.Number, tokens.Variable, tokens.CloseParen)
        ) and isinstance(_tokens[i + 1], tokens.OpenParen):
            _tokens.insert(i + 1, tokens.Multiply())
        elif isinstance(_tokens[i], tokens.CloseParen) and isinstance(
            _tokens[i + 1], (tokens.Number, tokens.Variable, tokens.OpenParen)
        ):
            _tokens.insert(i + 1, tokens.Multiply())

        i += 1
    return _tokens


def parse_expression(parser, expression):
    _tokens = parser.parse(expression)
    _tokens = preprocess_tokens(_tokens)
    return _tokens


def evaluate_tokenized_expression(env, _tokens):
    tree_builder = binary_tree.TreeBuilder()
    tree_builder.build_tree(_tokens)
    tree = tree_builder.get_tree()
    env.update(tree_builder.get_env())
    return tree.evaluate(env)


def main():
    global_env = {"g": 9.81, "feet_per_meter": 3.28084, "cm_per_inch": 2.54}
    history = []
    parser = lexical_analyzer.TokenParser()
    while True:
        try:
            expression = input("> ")
            if expression == "":
                break

            env = build_env_from_history(history)
            env.update(global_env)
            _tokens = parse_expression(parser, expression)
            result = evaluate_tokenized_expression(env, _tokens)
        except Exception as e:
            # raise e
            print(e)
            continue
        history.append(result)
        print(f"x{len(history)-1} = {result}")


if __name__ == "__main__":
    main()

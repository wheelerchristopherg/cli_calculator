#!/usr/bin/env python
import re
import tokens
import lexical_analyzer


class BinaryTree:
  def __init__(self, token, left=None, right=None):
    self.token = token
    self.left = left
    self.right = right

  def __str__(self):
    return self.token.__str__()

  def __repr__(self):
    return self.__str__()

  def _check_valid_number_token(self):
    if isinstance(self.token, tokens.Integer) or isinstance(self.token, tokens.Float): 
      if self.left or self.right:
        raise Exception("Invalid Tree")
      return self.token.evaluate()
    return None

  def _check_valid_var_token(self, env):
    if isinstance(self.token, tokens.Variable): 
      if self.left or self.right:
        raise Exception("Invalid Tree")
      return self.token.evaluate(env)
    return None

  def _check_valid_operation(self):
    if not (self.left and self.right and self.token):
      raise Exception(f"Invalid Operation: {self.token}({self.left}, {self.right})")

  def evaluate(self, env):
    if (valid_number := self._check_valid_number_token()) != None:
      return valid_number

    if (resolved_var := self._check_valid_var_token(env)) != None:
        return resolved_var

    self._check_valid_operation()
    return self.token.evaluate(self.left.evaluate(env), self.right.evaluate(env))


class TreeBuilder:
  def __init__(self):
    self.paren_substitutions = {}
    self.tree = None

  def _find_root(self, tokens):
    root_index = 0
    for i, t in enumerate(tokens):
      if t.weight >= tokens[root_index].weight:
        root_index = i
    return root_index

  def _substitute_paren_expressions(self, _tokens):
    if tokens.OpenParen in _tokens:
      if tokens.CloseParen not in _tokens:
        raise Exception("Invalid Expression: Missing )")
      first = _tokens.index(OpenParen)
      last = len(_tokens) - list(reversed(_tokens)).index(tokens.CloseParen)
      sub_expression = _tokens[first+1:last-1]
      next_var = f"p{len(self.paren_substitutions)}"
      self.paren_substitutions[next_var] = None
      self.paren_substitutions[next_var] = self._build_tree(sub_expression)
      return _tokens[:first] + [tokens.Variable(next_var)] + _tokens[last:]
      
    elif tokens.CloseParen in _tokens:
      raise Exception("Invalid Expression: Unexpected )")
    return _tokens

  def _build_tree(self, _tokens):
    if not _tokens:
      raise Exception("Invalid Expression")

    _tokens = self._substitute_paren_expressions(_tokens)

    if len(_tokens) == 1:
      return BinaryTree(_tokens[0])

    root_index = self._find_root(_tokens)
    left_node = self._build_tree(_tokens[:root_index])
    right_node = self._build_tree(_tokens[root_index+1:])
    return BinaryTree(_tokens[root_index], left_node, right_node)

  def build_tree(self, _tokens):
    self.tree = self._build_tree(_tokens)

  def get_tree(self):
    return self.tree

  def get_env(self):
    return self.paren_substitutions


#def token_factory(text):
#  if Number.is_number(text):
#    return Number(text)
#  elif text == "*":
#    return Multiply()
#  elif text == "/":
#    return Divide()
#  elif text == "+":
#    return Add()
#  elif text == "-":
#    return Subtract()
#  elif text == "(":
#    return OpenParen
#  elif text == ")":
#    return CloseParen
#  elif text != "":
#    return Variable(text)
#  raise Exception("Invalid token")


#def parse_tokens(expression):
#  tokens = list()
#  for t in expression.split():
#    tokens.append(token_factory(t))
#  return tokens


def build_env_from_history(history):
  return {f"x{i}" : value for i, value in enumerate(history)}  


def main():
  history = []
  while True:
    try:
      expression = input("> ")
      if expression == "":
        break

      env = build_env_from_history(history)
      tokens = lexical_analyzer.parse_tokens(expression)
      tree_builder = TreeBuilder()
      tree_builder.build_tree(tokens)
      tree = tree_builder.get_tree()
      env.update(tree_builder.get_env())
      result = tree.evaluate(env)
    except Exception as e:
      print(e)
      continue
    history.append(result)
    print(f"x{len(history)-1} = {result}")


if __name__ == "__main__":
  main()


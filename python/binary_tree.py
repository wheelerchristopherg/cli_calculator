import tokens


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
    if isinstance(self.token, tokens.Number): 
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
    if not (self.left and self.right and self.token and isinstance(self.token, tokens.Operator)):
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
      first = _tokens.index(tokens.OpenParen)
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



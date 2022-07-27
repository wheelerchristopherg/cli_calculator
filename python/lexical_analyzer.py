import tokens


DIGIT = "digit"
ALPHA = "alpha"


class UnexpectedCharacter(Exception):
  pass


class NoTokenMatched(Exception):
  pass


class State:
  @staticmethod
  def get_character_type(character):
    if character.isdigit():
      return DIGIT
    elif character.isalpha():
      return ALPHA
    return None

  def __init__(self, matched_token_type=None):
    self.transitions = {}
    self.matched_token_type = matched_token_type

  def add_transition(self, character, state):
    self.transitions.update({character: state})

  def next_state(self, character):
    if character in self.transitions:
      return self.transitions[character]
    elif (char_type := State.get_character_type(character)) in self.transitions:
      return self.transitions[char_type]
    elif self.matched_token_type:
      return self.matched_token_type

    if character:
      raise UnexpectedCharacter(character)
    raise NoTokenMatched()
         

def initialize_state_machine():
  states = []
  states.append(State())
  states.append(State(tokens.Integer))
  states.append(State())
  states.append(State(tokens.OpenParen))
  states.append(State(tokens.CloseParen))
  states.append(State(tokens.Minus))
  states.append(State(tokens.Plus))
  states.append(State(tokens.Multiply))
  states.append(State(tokens.Divide))
  states.append(State(tokens.Float))
  states.append(State(tokens.EOL))
  states.append(State(tokens.WhiteSpace))

  states[0].add_transition(DIGIT, states[1])
  states[0].add_transition('.', states[2])
  states[0].add_transition('(', states[3])
  states[0].add_transition(')', states[4])
  states[0].add_transition('-', states[5])
  states[0].add_transition('+', states[6])
  states[0].add_transition('*', states[7])
  states[0].add_transition('/', states[8])
  states[0].add_transition('\n', states[10])
  states[0].add_transition(' ', states[11])
  states[0].add_transition('\t', states[11])

  states[1].add_transition(DIGIT, states[1])
  states[1].add_transition('.', states[2])

  states[2].add_transition(DIGIT, states[9])
  
  states[9].add_transition(DIGIT, states[9])

  states[11].add_transition(' ', states[11])
  states[11].add_transition('\t', states[11])

  return states
  

def parse_tokens(_input):
  text = _input + "\n"
  states = initialize_state_machine()
  current_state = states[0]
  token_start = 0
  char_pos = 0
  token_list = []

  while char_pos < len(text):
    value = None
    try:
      value = current_state.next_state(text[char_pos])
    except UnexpectedCharacter as e:
      error_position = " " * char_pos
      error_position += "^"
      mono_char_text = text.replace('\t', ' ')
      raise UnexpectedCharacter(f"Unexpected character {e.args[0]} at position {char_pos}\n{mono_char_text[:-1]}\n{error_position}")
    except NoTokenMatched as e:
      raise e

    if value is tokens.EOL:
      token_list.append(value())
      break

    if type(value) is type(tokens.Token) and issubclass(value, tokens.Token):
      current_state = states[0]
      token_text = text[token_start:char_pos]
      new_token = None
      if value is tokens.Integer or value is tokens.Float:
        new_token = value(token_text)
      else:
        new_token = value()
      token_list.append(new_token)
      token_start = char_pos
    elif isinstance(value, State):
      current_state = value
      char_pos += 1 
  
  return list(filter(lambda x: not isinstance(x, tokens.WhiteSpace), token_list))
    

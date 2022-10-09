import tokens


DIGIT = "digit"
ALPHA = "alpha"


class UnexpectedCharacter(Exception):
    pass


class State:
    def __init__(self, matched_token_type=None):
        self.transitions = {}
        self.matched_token_type = matched_token_type

    def add_transition(self, character, state):
        self.transitions.update({character: state})

    def get_next_state(self, character):
        char_type = State.get_character_type(character)
        if character in self.transitions:
            return self.transitions[character]
        elif char_type in self.transitions:
            return self.transitions[char_type]
        elif self.matched_token_type:
            return None

        if character:
            raise UnexpectedCharacter(character)

    def get_token(self):
        return self.matched_token_type

    @staticmethod
    def get_character_type(character):
        if not isinstance(character, str):
            return None
        if character.isdigit():
            return DIGIT
        elif character.isalpha():
            return ALPHA
        return None


class TokenParser:
    def __init__(self):
        self.initialize_state_machine()

    def initialize_state_machine(self):
        self.states = []
        self.states.append(State())
        self.states.append(State(tokens.Integer))
        self.states.append(State())
        self.states.append(State(tokens.OpenParen))
        self.states.append(State(tokens.CloseParen))
        self.states.append(State(tokens.Minus))
        self.states.append(State(tokens.Plus))
        self.states.append(State(tokens.Multiply))
        self.states.append(State(tokens.Divide))
        self.states.append(State(tokens.Float))
        self.states.append(State(tokens.EOL))
        self.states.append(State(tokens.WhiteSpace))
        self.states.append(State(tokens.Variable))

        self.states[0].add_transition(DIGIT, self.states[1])    # integer
        self.states[0].add_transition(".", self.states[2])      # decimal start
        self.states[0].add_transition("(", self.states[3])      # open paren
        self.states[0].add_transition(")", self.states[4])      # close paren
        self.states[0].add_transition("-", self.states[5])      # minus
        self.states[0].add_transition("+", self.states[6])      # plus
        self.states[0].add_transition("*", self.states[7])      # multiply
        self.states[0].add_transition("/", self.states[8])      # divide
        self.states[0].add_transition("\n", self.states[10])    # EOL
        self.states[0].add_transition(" ", self.states[11])     # whitespace
        self.states[0].add_transition("\t", self.states[11])    # whitespace
        self.states[0].add_transition(ALPHA, self.states[12])   # variable
        self.states[0].add_transition("_", self.states[12])     # variable

        self.states[1].add_transition(DIGIT, self.states[1])    # integer
        self.states[1].add_transition(".", self.states[2])      # decimal start

        self.states[2].add_transition(DIGIT, self.states[9])    # float

        self.states[9].add_transition(DIGIT, self.states[9])    # float

        self.states[11].add_transition(" ", self.states[11])    # whitespace
        self.states[11].add_transition("\t", self.states[11])   # whitespace

        self.states[12].add_transition(ALPHA, self.states[12])  # variable
        self.states[12].add_transition(DIGIT, self.states[12])  # variable
        self.states[12].add_transition("_", self.states[12])    # variable

    def parse(self, _input):
        text = _input + "\n"
        current_state = self.states[0]
        token_start = 0
        char_pos = 0
        token_list = []

        while char_pos < len(text):
            try:
                next_state = current_state.get_next_state(text[char_pos])
            except UnexpectedCharacter as e:
                error_position = " " * char_pos
                error_position += "^"
                mono_char_text = text.replace("\t", " ")
                raise UnexpectedCharacter(
                    "Unexpected character {} at position {}\n{}\n{}".format(
                        e.args[0], char_pos + 1, mono_char_text[:-1], error_position
                    )
                )

            value = None
            if not next_state:
                value = current_state.get_token()
                current_state = self.states[0]
            else:
                current_state = next_state
                char_pos += 1
                continue

            if value is tokens.EOL:
                token_list.append(value())
                break

            if value and issubclass(value, tokens.Token):
                token_text = text[token_start:char_pos]
                new_token = None
                if issubclass(value, tokens.Number) or issubclass(
                    value, tokens.Variable
                ):
                    new_token = value(token_text)
                else:
                    new_token = value()
                if not isinstance(new_token, tokens.WhiteSpace):
                    token_list.append(new_token)
                token_start = char_pos

        return token_list

NAME=npuzzle
SOURCE=$()

all: $(NAME)

$(NAME): $(wildcard srcs/*.rs)
	rustc srcs/main.rs -o $(NAME)

fclean:
	rm $(NAME)

re: fclean all

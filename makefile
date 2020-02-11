NAME		= npuzzle

SRC		= main
SRC		+= parsing
SRC		+= map
SRC		+= map_procedure
SRC		+= make_final_grid
SRC		+= is_doable
SRC		+= heuristics

SRCS_DIR	= ./srcs/

SRCS		=$(addprefix $(SRCS_DIR), $(addsuffix .rs, $(SRC)))

CC		= rustc

.PHONY: all fclean re

all: $(NAME)

$(NAME): $(SRCS)
	$(CC) $(SRCS_DIR)main.rs -o $(NAME)

fclean:
	$(RM) $(NAME)

re: fclean all

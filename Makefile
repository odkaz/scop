NAME := scop

$(NAME): all

all:
	cargo run

clean:
	cargo clean

fclean: clean

re: fclean all

.PHONY: all clean fclean re

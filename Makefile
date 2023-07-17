NAME := scop

$(NAME): all

all:
	cargo run "resources/obj/dragon.obj"

clean:
	cargo clean

fclean: clean

re: fclean all

.PHONY: all clean fclean re

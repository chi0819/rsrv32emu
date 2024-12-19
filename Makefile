TOOLCHAIN = riscv64-unknown-elf

ASM_DIR = asm
OBJ_DIR = obj
BIN_DIR = bin
LINKER_DIR = linker

ASM_FILES := $(wildcard $(ASM_DIR)/*.s)
OBJ_FILES := $(patsubst $(ASM_DIR)/%.s, $(OBJ_DIR)/%.o, $(ASM_FILES))
ELF_FILES := $(patsubst $(ASM_DIR)/%.s, $(OBJ_DIR)/%.elf, $(ASM_FILES))
BIN_FILES := $(patsubst $(ASM_DIR)/%.s, $(BIN_DIR)/%.bin, $(ASM_FILES))

all: $(BIN_FILES)

$(OBJ_FILES): $(ASM_FILES)
	mkdir -p $(OBJ_DIR)
	$(TOOLCHAIN)-as -march=rv32i -mabi=ilp32 -o $@ $<

$(ELF_FILES): $(OBJ_FILES)
	$(TOOLCHAIN)-ld -m elf32lriscv -T $(LINKER_DIR)/linker.ld -o $@ $<
	rm $(OBJ_FILES)

$(BIN_FILES): $(ELF_FILES)
	mkdir -p $(BIN_DIR)
	$(TOOLCHAIN)-objcopy -O binary $< $@

clean:
	rm -rf $(OBJ_DIR) $(BIN_DIR)

run:
	cargo run $1 $2

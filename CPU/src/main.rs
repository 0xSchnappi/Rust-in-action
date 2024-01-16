/*
 * 1.初始化CPU
 * 2.把u8的值加载到registers(寄存器)中。
 * 3.把加法运算的操作码加载到current_operation(当前的操作码)中
 * 4.执行此加法运算的操作中
 */
struct CPU1 {
    current_operation: u16, // CHIP-8的所有操作码都是u16类型的
    registers: [u16; 2],    // 对于加法运算来说，这两个寄存器就够了
}

// CPU 加法操作
impl CPU1 {
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    fn run(&mut self) {
        // 执行操作码
        // loop {
        let opcode = self.read_opcode();

        /*
         * CHIP-8 三种主要形式
         * 0xcxyd
         * 第一种：c操作码 x寄存器 kk参数
         * 第二种：c操作码 nnn地址
         * 第三种：c操作码 x寄存器 y寄存器 操作码子类型
         */
        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = (opcode & 0x000F) as u8;

        let nnn = opcode & 0x0FFF;
        let kk = opcode & 0x00FF;

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:04x}", opcode),
        }
        // }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

struct CPU2 {
    registers: [u8; 16],
    position_in_memory: usize, // 类似与eip/rip寄存器(程序计数器)，指向下一条指令地址
    memory: [u8; 4096],
}

// CPU累加操作
impl CPU2 {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        (op_byte1 << 8) | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = (opcode & 0x000F) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

/*
 * 函数调用栈
 * call操作码 0x2nnn
 * 函数返回操作码 0x00EE
 */

struct CPU3 {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize,
}

/*
 * 函数调用步骤
 * 1.在栈上保存当前的内存位置
 * 2.自增栈指针
 * 3.把当前内存位置设置为预期的内存地址
 * 函数返回的过程与调用过程是相反的
 * 1.自减栈指针
 * 2.从栈中取回调用前的内存地址
 * 3.把当前内存位置设置为预期的内存地址
 */
impl CPU3 {

    fn read_opcode(&mut self) -> u16{
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        (op_byte1 << 8) | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000)>>12) as u8;
            let x = ((opcode & 0x0F00)>>8) as u8;
            let y = ((opcode & 0x00F0)>>4 ) as u8;
            let d = ((opcode & 0x000F)) as u8;

            let nnn = opcode & 0x0FFF;
            let kk = (opcode & 0x00FF) as u8;

            match (c, x, y, d) {
                (0,0,0,0) => {return;},
                (0,0,0xE,0xE) => self.ret(),
                (0x2,_,_,_) =>self.call(nnn),
                (0x8,_,_,_0x4) => self.add_xy(x,y),
                _ =>todo!("opcode{:04x}",opcode),
            }
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {   // 对栈进行安全性检查
            panic!("stack overflow");
        }

        stack[sp] = self.position_in_memory as u16;     // 将下一条指令入栈，保存返回地址
        self.stack_pointer += 1;    // 提升栈指针
        self.position_in_memory = addr as usize;    // 将程序计数器改为函数地址
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("stack underflow");
    }
    self.stack_pointer -= 1;    // 清除栈指针
    let addr = self.stack[self.stack_pointer];  // 获取函数返回地址
    self.position_in_memory = addr as usize;    // 将程序计数器改为函数返回地址
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow_detected) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow_detected {
            self.registers[0xF] = 1;
            
        }
        else {
            self.registers[0xF] = 0;
        }
    }
}

fn main() {
    let mut cpu1 = CPU1 {
        current_operation: 0,
        registers: [0; 2],
    };
    // 加载u8的值到registers中
    cpu1.registers[0] = 5;
    cpu1.registers[1] = 10;
    cpu1.current_operation = 0x8014; // 加法操作码

    cpu1.run();

    assert_eq!(cpu1.registers[0], 15); // 断言结果

    println!("5 + 10 = {}", cpu1.registers[0]);

    let mut cpu2 = CPU2 {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };
    cpu2.registers[0] = 5;
    cpu2.registers[1] = 10;
    cpu2.registers[2] = 10;
    cpu2.registers[3] = 10;

    let mem = &mut cpu2.memory;
    mem[0] = 0x80;
    mem[1] = 0x14; // 加法操作码
    mem[2] = 0x80;
    mem[3] = 0x24; // 加法操作码
    mem[4] = 0x80;
    mem[5] = 0x34;

    cpu2.run();
    assert_eq!(cpu2.registers[0], 35); // 断言结果
    println!("5 + 10 + 10 + 10 = {}", cpu2.registers[0]);

    let mut memory: [u8; 4096] = [0; 4096];
    let mem3 = &mut memory;
    /*
     * 定义一个加法函数
     * add_twice:
     *      0x8014
     *      0x8014
     *      0x00EE
     */
    // let add_twice:[u16;3] = [0x8014,0x8014,0x00EE];
    let add_twice: [u8; 6] = [0x80, 0x14, 0x80, 0x14, 0x00, 0xEE];
    mem3[0x100..0x106].copy_from_slice(&add_twice);
    println!("{:?}", &memory[0x100..0x106]);

    let mut cpu3 = CPU3 {
        registers:[0;16],
        memory:[0;4096],
        position_in_memory:0,
        stack:[0;16],
        stack_pointer:0,
    };

    cpu3.registers[0] = 5;
    cpu3.registers[1] = 10;

    let mem = &mut cpu3.memory;
    mem[0x000] = 0x21; mem[0x001] = 0x00;
    mem[0x002] = 0x21; mem[0x003] = 0x00;
    mem[0x004] = 0x00; mem[0x005] = 0x00;

    mem[0x100] = 0x80; mem[0x101] = 0x14; // 加法操作码
    mem[0x102] = 0x80; mem[0x103] = 0x14; //
    mem[0x104] = 0x00; mem[0x105] = 0xEE; //

    cpu3.run();

    assert_eq!(cpu3.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu3.registers[0]);

}

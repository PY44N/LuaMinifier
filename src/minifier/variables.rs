use super::util::spinlock::SpinLock;

static CHARS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

struct UsedVariables {
    pub used_variables_key: Vec<String>,
    pub used_variables_value: Vec<String>,
}

struct VariableData {
    pub count: usize,
}

static VARIABLE_DATA: SpinLock<VariableData> = SpinLock::new(VariableData { count: 0 });
static USED_VARIABLES: SpinLock<UsedVariables> = SpinLock::new(UsedVariables {
    used_variables_key: Vec::new(),
    used_variables_value: Vec::new(),
});
static LOCALS: SpinLock<Vec<String>> = SpinLock::new(Vec::new());

fn generate_variable() -> String {
    let mut data = VARIABLE_DATA.lock();
    let mut count = data.count;
    let mut ret = String::new();

    ret.push(
        CHARS
            .chars()
            .nth(count % CHARS.len())
            .expect("Failed to get char"),
    );
    count /= CHARS.len();

    while count > 0 {
        ret.push(
            CHARS
                .chars()
                .nth(count % CHARS.len())
                .expect("Failed to get char"),
        );
        count /= CHARS.len();
    }

    data.count += 1;

    ret
}

pub fn get_variable(var: String) -> String {
    let mut data = USED_VARIABLES.lock();
    let locals = LOCALS.lock();

    if !locals.contains(&var) {
        return var;
    }

    if data.used_variables_key.contains(&var) {
        data.used_variables_value[data
            .used_variables_key
            .iter()
            .position(|v| *v == var)
            .unwrap()]
        .clone()
    } else {
        let var_out = generate_variable();
        data.used_variables_key.push(var);
        data.used_variables_value.push(var_out.clone());
        var_out
    }
}

pub fn load_locals(locals: Vec<String>) {
    let mut locals_list = LOCALS.lock();

    for local in locals {
        locals_list.push(local);
    }
}

def is_valid_vector(s: str) -> bool:
    return len(s) in (2, 4, 8) and all(c in '01' for c in s)


def preserves_zero(vec):
    return vec[0] == 0


def preserves_one(vec):
    return vec[-1] == 1


def is_self_dual(vec):
    n = len(vec)
    for i in range(n):
        if vec[i] == vec[n - 1 - i]:
            return False
    return True


def is_monotone(vec):
    n = len(vec)
    for i in range(n):
        for j in range(n):
            if (i & j) == i:   # i ≤ j
                if vec[i] > vec[j]:
                    return False
    return True


def is_linear(vec):
    coeff = vec[:]
    n = len(coeff)

    step = 1
    while step < n:
        for i in range(n):
            if i & step:
                coeff[i] ^= coeff[i ^ step]
        step <<= 1

    for i in range(n):
        if coeff[i] == 1:
            if i != 0 and (i & (i - 1)) != 0:
                return False
    return True


def analyze_function(vec):
    return {
        "T0": preserves_zero(vec),
        "T1": preserves_one(vec),
        "L": is_linear(vec),
        "M": is_monotone(vec),
        "S": is_self_dual(vec)
    }


def is_complete(functions):
    flags = {"T0": False, "T1": False, "L": False, "M": False, "S": False}

    for f in functions:
        for key in flags:
            if not f[key]:
                flags[key] = True

    return all(flags.values())


def is_basis(functions):
    if not is_complete(functions):
        return False

    for i in range(len(functions)):
        temp = functions[:i] + functions[i+1:]
        if temp and is_complete(temp):
            return False

    return True


n = int(input("Введите количество функций: "))
vectors = []


for i in range(n):
    while True:
        v = input(f"Введите вектор {i+1}: ")
        if is_valid_vector(v):
            vectors.append([int(c) for c in v])
            break
        else:
            print("Ошибка: длина должна быть 2, 4 или 8 и состоять только из 0 и 1")


results = [analyze_function(v) for v in vectors]


print("\nТаблица принадлежности:")
print(f"{'Вектор':<10}{'T0':^4}{'T1':^4}{'L':^4}{'M':^4}{'S':^4}")
print("-" * 29)


for v, r in zip(vectors, results):
    vector_str = "".join(map(str, v))
    print(
        f"{vector_str:<10}"
        f"{('+' if r['T0'] else '-'):^4}"
        f"{('+' if r['T1'] else '-'):^4}"
        f"{('+' if r['L'] else '-'):^4}"
        f"{('+' if r['M'] else '-'):^4}"
        f"{('+' if r['S'] else '-'):^4}"
    )


print("\nПолнота системы:", "ДА" if is_complete(results) else "НЕТ")
print("Является базисом:", "ДА" if is_basis(results) else "НЕТ")
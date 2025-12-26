from itertools import combinations

SDNF = None
MIN_DNF_LIST = []

def validate_input(bitstring: str) -> bool:
    return len(bitstring) == 16 and all(c in "01" for c in bitstring)

def get_valid_input() -> str:
    while True:
        s = input("Введите 16-битную строку (0/1): ").strip()
        if validate_input(s):
            return s
        print("Ошибка ввода. Попробуйте снова.")

def build_sdnf(bitstring: str):
    vars_names = ['x', 'y', 'z', 'w']
    unique_terms = set()
    for i in range(16):
        if bitstring[i]=='1':
            bits=format(i,'04b')
            literals=[var if bits[j]=='1' else var.upper() for j,var in enumerate(vars_names)]
            unique_terms.add(tuple(literals))
    return [list(term) for term in unique_terms]

def sdnf_to_string(sdnf):
    if not sdnf: return "0"
    return " | ".join("(" + " & ".join(term) + ")" for term in sdnf)

def print_truth_table(bitstring:str):
    print("\nТаблица истинности (x y z w → f):")
    print(" x y z w | f")
    print("---------+--")
    for i in range(16):
        x,y,z,w = format(i,'04b')
        print(f" {x} {y} {z} {w} | {bitstring[i]}")

def count_ones(bits:str): return bits.count('1')

def combine_terms(term1, term2):
    diff_count=0
    new_term=[]
    for a,b in zip(term1,term2):
        if a==b: new_term.append(a)
        else:
            diff_count+=1
            new_term.append('-')
        if diff_count>1: return None
    return tuple(new_term)

def matches_bin(term,bits):
    return all(t==b or t=='-' for t,b in zip(term,bits))

def covered_minterms(term,minterms):
    return [m for m in minterms if matches_bin(term,format(m,'04b'))]

def term_to_literals(term):
    var_names=['x','y','z','w']
    literals=[]
    for i,val in enumerate(term):
        if val=='1': literals.append(var_names[i])
        elif val=='0': literals.append(var_names[i].upper())
    return literals

def minterm_to_string(i):
    bits=format(i,'04b')
    var_names=['x','y','z','w']
    return ''.join([var_names[j] if b=='1' else var_names[j].upper() for j,b in enumerate(bits)])

def quine_mccluskey(sdnf, bitstring):
    bin_terms=[]
    minterms=[i for i,b in enumerate(bitstring) if b=='1']
    for term in sdnf:
        b=''.join('1' if lit.islower() else '0' for lit in term)
        bin_terms.append(tuple(b))

    all_terms=set(bin_terms)
    prime_implicants=set()
    step=1
    print("\n--- Этапы склеек ---")
    while all_terms:
        grouped={}
        for t in all_terms:
            ones=count_ones(''.join(t))
            grouped.setdefault(ones,[]).append(t)

        new_terms=set()
        used=set()
        step_output=[]
        for i in range(5):
            g1=grouped.get(i,[])
            g2=grouped.get(i+1,[])
            for a in g1:
                for b in g2:
                    combined=combine_terms(a,b)
                    if combined:
                        new_terms.add(combined)
                        used.add(a)
                        used.add(b)
                        step_output.append(f"{''.join(a)} + {''.join(b)} -> {''.join(combined)}")
        if step_output:
            print(f"\nСклейки шаг {step}:")
            for line in step_output: print(line)
        for t in all_terms:
            if t not in used: prime_implicants.add(t)
        if not new_terms: break
        all_terms=new_terms
        step+=1

    prime_literals=[]
    coverage_table=[]
    for term in prime_implicants:
        literals=term_to_literals(term)
        prime_literals.append(literals)
        covered=covered_minterms(term,minterms)
        coverage_table.append((literals,covered))

    return prime_literals,coverage_table,minterms

def find_mdnf(coverage_table,minterms):
    implicants=[lit for lit,_ in coverage_table]
    covered_sets=[set(cov) for _,cov in coverage_table]
    solutions=[]
    for r in range(1,len(implicants)+1):
        for comb in combinations(range(len(implicants)),r):
            covered=set()
            for idx in comb:
                covered |= covered_sets[idx]
            if covered==set(minterms):
                solutions.append([implicants[i] for i in comb])
        if solutions: break
    return solutions

def print_coverage_table(coverage_table,minterms):
    print("\nТаблица покрытия:")
    minterm_names=[minterm_to_string(m) for m in minterms]
    col_width=max(max(len("".join(lit)) for lit,_ in coverage_table), max(len(name) for name in minterm_names))+2

    header=["".ljust(col_width)]+[name.center(col_width) for name in minterm_names]
    print("".join(header))

    for lit,covered in coverage_table:
        row_name="".join(lit).ljust(col_width)
        row=[row_name]
        for m in minterms:
            row.append(('+' if m in covered else '-').center(col_width))
        print("".join(row))

def main():
    global SDNF,MIN_DNF_LIST
    bitstring=get_valid_input()
    SDNF=build_sdnf(bitstring)
    print("\nСДНФ:")
    print(sdnf_to_string(SDNF))
    print_truth_table(bitstring)

    prime_literals,coverage_table,minterms=quine_mccluskey(SDNF,bitstring)
    MIN_DNF_LIST=find_mdnf(coverage_table,minterms)

    print_coverage_table(coverage_table,minterms)

    for i,mdnf in enumerate(MIN_DNF_LIST,1):
        print(f"\nМДНФ{i}: "+" | ".join("("+" & ".join(term)+")" for term in mdnf))

if __name__=="__main__":
    main()
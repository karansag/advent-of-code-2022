from os import path

def process_parsed(lines):
    base_node = {'name': '/', 'files': [], 'parent': None, 'children': [], 'path': '/'}
    current_node = base_node
    for l in lines:
        if l[0] == '$':
            if l[1] == 'cd':
                if l[2] == '/':
                   current_node = base_node
                elif l[2] == '..':
                    current_node = current_node['parent']
                else:
                    next_dir = l[2]
                    next_node = [x for x in current_node['children'] if x['name'] == next_dir][0]
                    current_node = next_node
            elif l[1] == 'ls':
                pass
        elif l[0] == 'dir':
            name = l[1]
            if name not in [x['name'] for x in current_node['children']]:
                current_node['children'].append(
                    {'name':name, 'files': [], 'parent': current_node, 'children': [], 'path': path.join(current_node['path'], name)}
                )
        else:
            size = int(l[0])
            name = l[1]
            if name not in [x['name'] for x in current_node['files']]:
                current_node['files'].append({
                    'name': name,
                    'size': size,
                })

    return base_node

def parse_lines(filename: str):
    with open(filename, 'r') as f:
        lines = f.readlines()

    return [x.strip().split(' ') for x in lines]

def calc_sizes(base_node):
    memo = {}


    def _calc(n):
        if n['path'] in memo:
            return memo[n['path']]
        file_tot = sum(map(lambda x: x['size'], n['files']))
        dir_tot = sum(_calc(x) for x in n['children'])
        memo[n['path']] = file_tot + dir_tot
        return file_tot + dir_tot
    _calc(base_node)
    return memo




def main():
    parsed = parse_lines('../inputs/day-7-input.txt')

    base_node = process_parsed(parsed)
    sizes = calc_sizes(base_node)
    print(sum(size for (path, size) in sizes.items() if size <= 100000))
    total_usage = sizes['/']
    total_to_be_freed = 30000000 - (70000000 - total_usage)
    print('total needed to be freed: ', total_to_be_freed)
    x = sorted(size for (path, size) in sizes.items() if size > total_to_be_freed)[0]
    print(x)




if __name__ == '__main__':
    main()

import os
import json

def path_to_dict(path):
    d = {'name': os.path.basename(path)}
    if os.path.isdir(path):
        d['type'] = 'folder'
        # Sort the children to have a consistent order
        children = sorted(os.listdir(path))
        d['children'] = [path_to_dict(os.path.join(path, x)) for x in children]
    else:
        d['type'] = 'file'
    return d

# Create a representation of the docs directory, but start from its contents
docs_root = 'docs'
docs_structure = {
    'name': docs_root,
    'type': 'folder',
    'children': [path_to_dict(os.path.join(docs_root, x)) for x in sorted(os.listdir(docs_root))]
}


with open('docs/file_list.js', 'w') as f:
    f.write('const fileTree = ')
    json.dump(docs_structure, f, indent=4)
    f.write(';')

print("Successfully generated docs/file_list.js")

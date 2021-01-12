import os
import tempfile
import subprocess
from pathlib import Path

if not 'DEVKITPRO' in os.environ:
    print('DEVKITPRO not found in environment variables')
    quit(-1)

dkp_root = os.environ['DEVKITPRO']

# Allow overriding of path to wut outside of devkitPro incase you have a local
# copy of wut with different header files.
if not 'WUT_ROOT' in os.environ:
    wut_root = os.path.join(dkp_root, 'wut')
else:
    wut_root = os.environ['WUT_ROOT']

include_subdirs = [
    'avm',
    'coreinit',
    'dmae',
    'gx2',
    'gx2r',
    'h264',
    'nn',
    'nsyshid',
    'nsysnet',
    'padscore',
    'proc_ui',
    'sndcore2',
    'swkbd',
    'sysapp',
    'vpad',
]

sysroot = os.path.join(dkp_root, 'devkitPPC', 'powerpc-eabi')

gcc_include_path = os.listdir(os.path.join(dkp_root, 'devkitPPC', 'lib', 'gcc', 'powerpc-eabi'))
if len(gcc_include_path) != 1:
    print('Unexpected include paths in devkitPPC')
    quit(-1)
gcc_include_path = os.path.join(dkp_root, 'devkitPPC', 'lib', 'gcc', 'powerpc-eabi', gcc_include_path[0])

tf = tempfile.NamedTemporaryFile(mode='w', suffix='.h',delete=False)
for subdir in include_subdirs:
    for path in Path(os.path.join(wut_root, 'include', subdir)).rglob('*.h'):
        tf.write('#include "%s"\n' % (path))
tf.close()
print(tf.name)

cmd = [
    'bindgen',
    tf.name,
    '--use-core',
    '--ctypes-prefix=cty',
    '--default-enum-style', 'moduleconsts',
    '--no-include-path-detection',
    '--raw-line', '#[allow(non_camel_case_types,non_snake_case,non_upper_case_globals,dead_code)]',
    '--blacklist-item=OSSpinLock',
    '--',
    '-target', 'powerpc-none-eabi',
    '-march=powerpc',
    '-mfloat-abi=hard',
    '-nostdinc',
    '--sysroot', sysroot,
    '-isystem', os.path.join(sysroot, 'include'),
    '-isystem', os.path.join(gcc_include_path, 'include'),
    '-isystem', os.path.join(gcc_include_path, 'include-fixed'),
    '-I%s' % os.path.join(wut_root, 'include'),
]
output = subprocess.check_output(cmd)

f = open(os.path.join('src', 'sys.rs'), 'wb')
f.write(output)
f.close()

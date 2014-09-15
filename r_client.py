#!/usr/bin/env python2
from xml.etree.cElementTree import *
from os.path import basename
from functools import reduce
import getopt
import os
import sys
import errno
import time
import re

# Jump to the bottom of this file for the main routine

# Some hacks to make the API more readable, and to keep backwards compability
_cname_re = re.compile('([A-Z0-9][a-z]+|[A-Z0-9]+(?![a-z])|[a-z]+)')
_cname_special_cases = {'DECnet':'decnet'}

_extension_special_cases = ['XPrint', 'XCMisc', 'BigRequests']

_cplusplus_annoyances = { 'new'   : 'new_', 'str'   : 'str_' }

_c_keywords = {'type' : 'type_', "str" : "str_"}

_hlines = []
_hlevel = 0
_rlines = []
_rlevel = 0
_ns = None

_imports = []

outdir = './'

# global variable to keep track of serializers and 
# switch data types due to weird dependencies
finished_serializers = []
finished_sizeof = []
finished_switch = []

# keeps enum objects so that we can refer to them when generating manpages.
enums = {}

links = {"composite":"composite",
         "damage":"damage",
         "dpms":"dpms",
         "dri2":"dri2",
         "glx":"glx",
         "randr":"randr",
         "record":"record",
         "render":"render",
         "res":"res",
         "screensaver":"screensaver",
         "shape":"shape",
         "shm":"shm",
         "sync":"sync",
         "xevie":"xevie",
         "xf86dri":"xf86dri",
         "xfixes":"xfixes",
         "xinerama":"xinerama",
         "xinput":"xinput",
         "xkb":"xkb",
         "xprint":"xprint",
         "xselinux":"xselinux",
         "xtest":"xtest",
         "xv":"xv",
         "xvmc":"xvmc"}

manpaths = False

def _h(fmt, *args):
    '''
    Writes the given line to the header file.
    '''
    _hlines[_hlevel].append(fmt % args)
    
def _r(fmt, *args):
    '''
    Writes the given line to the source file.
    '''
    _rlines[_rlevel].append(fmt % args)
    pass
    
def _hr(fmt, *args):
    '''
    Writes the given line to both the header and source files.
    '''
    _h(fmt, *args)
    _r(fmt, *args)

# XXX See if this level thing is really necessary.
def _h_setlevel(idx):
    '''
    Changes the array that header lines are written to.
    Supports writing different sections of the header file.
    '''
    global _hlevel
    while len(_hlines) <= idx:
        _hlines.append([])
    _hlevel = idx
    
def _r_setlevel(idx):
    '''
    Changes the array that source lines are written to.
    Supports writing to different sections of the source file.
    '''
    global _rlevel
    while len(_rlines) <= idx:
        _rlines.append([])
    _rlevel = idx
    
def _n_item(str):
    '''
    Does C-name conversion on a single string fragment.
    Uses a regexp with some hard-coded special cases.
    '''
    if str in _cname_special_cases:
        return _cname_special_cases[str]
    else:
        split = _cname_re.finditer(str)
        name_parts = [match.group(0) for match in split]
        return '_'.join(name_parts)
    
def _r_item(str):
    split = _cname_re.finditer(str)
    name_parts = [match.group(0) for match in split]
    name_parts = [i[0].upper() + i[1:].lower() for i in name_parts]
    return ''.join(name_parts)

def _cpp(str):
    '''
    Checks for certain C++ reserved words and fixes them.
    '''
    if str in _cplusplus_annoyances:
        return _cplusplus_annoyances[str]
    elif str in _c_keywords:
        return  _c_keywords[str]
    else:
        return str

def _ext(str):
    '''
    Does C-name conversion on an extension name.
    Has some additional special cases on top of _n_item.
    '''
    if str in _extension_special_cases:
        return _n_item(str).lower()
    else:
        return str.lower()
    
def _n(list):
    '''
    Does C-name conversion on a tuple of strings.
    Different behavior depending on length of tuple, extension/not extension, etc.
    Basically C-name converts the individual pieces, then joins with underscores.
    '''
    if len(list) == 1:
        parts = list
    elif len(list) == 2:
        parts = [list[0], _n_item(list[1])]
    elif _ns.is_ext:
        parts = [list[0], _ext(list[1])] + [_n_item(i) for i in list[2:]]
    else:
        parts = [list[0]] + [_n_item(i) for i in list[1:]]
    return '_'.join(parts).lower()

def _rn(list):
    '''
    Does C-name conversion on a tuple of strings.
    Different behavior depending on length of tuple, extension/not extension, etc.
    Basically C-name converts the individual pieces, then joins with underscores.
    '''
    if len(list) == 1:
        parts = list
    elif len(list) == 2:
        parts = [_r_item(list[1])]
    elif _ns.is_ext:
        parts = [_r_item(i) for i in list[2:]]
    else:
        parts = [_r_item(i) for i in list[1:]]
    return ''.join(parts)

def _t(list):
    '''
    Does C-name conversion on a tuple of strings representing a type.
    Same as _n but adds a "_t" on the end.
    '''

    module = ''
    if len(list) == 1:
        parts = list
    elif _ns.is_ext:
        list = list[1:]
        ext = _ext(list[0])
        if ext in _imports:
            module = 'ffi::'+(_ext(list[0]) + '::')
            parts =  [_n_item(i) for i in list[1:]]
        elif ext == _ext(_ns.ext_name):
            parts = [_n_item(i) for i in list[1:]]
        else:
            module = 'ffi::xproto::'
            parts = [_n_item(i) for i in list]

    elif len(list) == 2:
        parts = [_n_item(list[1])]
    else:
        parts = [_n_item(i) for i in list[1:]]
    t = '_'.join(parts).lower()

    t = _cpp(t)

    return module.lower() + t

def _rty(list):
    module = ''
    if len(list) == 1:
        parts = list
    elif _ns.is_ext:
        list = list[1:]
        ext = list[0]
        if _ext(ext) in _imports:
            module = (list[0] + '::')
            parts =  [i for i in list[1:]]
        elif ext == _ns.ext_name:
            parts = [i for i in list[1:]]
        else:
            module = 'xproto::'
            parts = [i for i in list]

        parts = [_r_item(i) for i in parts]
    elif len(list) == 2:
        parts = [list[1]]
        parts = [_r_item(i) for i in parts]
    else:
        parts = [i for i in list[1:]]
        parts = [_r_item(i) for i in parts]
    t = ''.join(parts)

    t = _cpp(t)

    return module.lower() + t


def c_open(self):
    '''
    Exported function that handles module open.
    Opens the files and writes out the auto-generated comment, header file includes, etc.
    '''
    global _ns
    _ns = self.namespace
    _ns.c_ext_global_name = _n(_ns.prefix + ('id',))

    print("Generating %s" % _ns.ext_name)

    # Build the type-name collision avoidance table used by c_enum
    build_collision_table()

    _h_setlevel(0)
    _r_setlevel(0)

    _hr('/*')
    _hr(' * This file generated automatically from %s by r_client.py.', _ns.file)
    _hr(' * Edit at your peril.')
    _hr(' */')
    _hr('')

    _hr('//Make the compiler quiet')
    _hr('#![allow(unused_imports)]')
    _r('#![allow(unused_unsafe)]')
    _h('#![allow(non_camel_case_types)]')

    _hr('use std;')
    _hr('use libc::*;')
    _r('use std::{mem,num,ptr,str};')
    _r('use ffi::base::*;')

    _r('use base;')
    _r('use base::*;')
    _hr('use ffi;')
    _r('use ffi::%s::*;', _ns.header)

    _r('use std::option::Option;')
    _r('use std::iter::Iterator;')
    _r('')

    global _imports

    if _ns.is_ext:
        for (n, h) in self.imports:
            _h('use ffi::%s;', h)
            _r('use %s;', h)
            _imports.append(h)

        _h('')
        _h('pub static %s_MAJOR_VERSION : c_uint = %s;', _ns.ext_name.upper(), _ns.major_version)
        _h('pub static %s_MINOR_VERSION : c_uint = %s;', _ns.ext_name.upper(), _ns.minor_version)

def c_close(self):
    '''
    Exported function that handles module close.
    Writes out all the stored content lines, then closes the files.
    '''
    _h_setlevel(2)
    _hr('')

    global links

    # Write header file
    hfile = open('src/ffi/%s.rs' % (_ns.header,), 'w')
    level = 0
    for list in _hlines:
        if level == 1:
            if _ns.header in links:
                hfile.write("#[link(name=\"lxcb-%s\")]\n" % links[_ns.header])
            hfile.write("extern \"C\" {\n")
        if level == 2:
            hfile.write("}\n")
        for line in list:
            hfile.write(line)
            hfile.write('\n')
        level = level + 1
    hfile.close()

    rfile = open('src/%s.rs' % (_ns.header,), 'w')
    level = 0
    for list in _rlines:
        for line in list:
            rfile.write(line)
            rfile.write('\n')
        level = level + 1
    rfile.close()


def build_collision_table():
    global namecount
    namecount = {}

    for v in module.types.values():
        name = _t(v[0])
        namecount[name] = (namecount.get(name) or 0) + 1

def c_enum(self, name):
    '''
    Exported function that handles enum declarations.
    '''

    enums[name] = self

    tname = _t(name)
    if namecount[tname] > 1:
        tname = _t(name + ('enum',))

    _r_setlevel(0)
    _r('')
    _r('pub type %s = c_uint;//{', tname)

    count = len(self.values)

    val = 0
    for (enam, eval) in self.values:
        count = count - 1
        val = int(eval) if eval != '' else val + 1
        doc = ''
        if hasattr(self, "doc") and self.doc and enam in self.doc.fields:
            doc = '\n/** %s */\n    ' % self.doc.fields[enam]
        _r('    %spub static %s : %s = %s;', doc, _n(name + (enam,)).upper(), tname, val)

    _r('//}')

def _c_type_setup(self, name, postfix):
    '''
    Sets up all the C-related state by adding additional data fields to
    all Field and Type objects.  Here is where we figure out most of our
    variable and function names.

    Recurses into child fields and list member types.
    '''
    # Do all the various names in advance
    self.c_type = _t(name + postfix)
    self.r_type = _rty(name + postfix)
    self.c_wiretype = 'u8' if self.c_type == 'void' else self.c_type

    self.c_iterator_type = _t(name + ('iterator',))
    self.r_iterator_type = _rty(name + ('iterator',))
    self.c_next_name = _n(name + ('next',))
    self.c_end_name = _n(name + ('end',))

    self.c_request_name = _n(name)
    self.c_checked_name = _n(name + ('checked',))
    self.c_unchecked_name = _n(name + ('unchecked',))
    self.r_request_name = _rn(name)
    self.r_checked_name = _rn(name + ('checked',))
    self.r_unchecked_name = _rn(name + ('unchecked',))
    self.c_reply_name = _n(name + ('reply',))
    self.c_reply_type = _t(name + ('reply',))
    self.r_reply_type = _rty(name + ('reply',))
    self.c_cookie_type = _t(name + ('cookie',))
    self.r_cookie_type = _rty(name + ('cookie',))

    self.need_aux = False
    self.need_serialize = False
    self.need_sizeof = False

    self.c_aux_name = _n(name + ('aux',))
    self.c_aux_checked_name = _n(name + ('aux', 'checked'))
    self.c_aux_unchecked_name = _n(name + ('aux', 'unchecked'))
    self.r_aux_name = _rn(name + ('aux',))
    self.r_aux_checked_name = _rn(name + ('aux', 'checked'))
    self.r_aux_unchecked_name = _rn(name + ('aux', 'unchecked'))
    self.c_serialize_name = _n(name + ('serialize',))
    self.c_unserialize_name = _n(name + ('unserialize',))
    self.c_unpack_name = _n(name + ('unpack',))
    self.c_sizeof_name = _n(name + ('sizeof',))

    # special case: structs where variable size fields are followed by fixed size fields
    self.var_followed_by_fixed_fields = False

    if self.is_switch:
        self.need_serialize = True
        self.c_container = 'struct'
        for bitcase in self.bitcases:
            bitcase.c_field_name = _cpp(bitcase.field_name)
            bitcase_name = bitcase.field_type if bitcase.type.has_name else name
            _c_type_setup(bitcase.type, bitcase_name, ())

    elif self.is_container:

        self.c_container = 'struct /* union */' if self.is_union else 'struct'
        prev_varsized_field = None
        prev_varsized_offset = 0
        first_field_after_varsized = None

        for field in self.fields:
            _c_type_setup(field.type, field.field_type, ())
            if field.type.is_list:
                _c_type_setup(field.type.member, field.field_type, ())
                if (field.type.nmemb is None): 
                    self.need_sizeof = True

            field.c_field_type = _t(field.field_type)
            field.r_field_type = _rty(field.field_type)
            field.c_field_const_type =  field.c_field_type
            field.r_field_const_type =  field.r_field_type
            field.c_field_name = _cpp(field.field_name)
            field.c_subscript = field.type.nmemb if (field.type.nmemb and field.type.nmemb > 1) else 1
            field.c_pointer = ' ' if field.type.nmemb == 1 else '*mut '
            field.r_pointer = ' ' if field.type.nmemb == 1 else '&'

            # correct the c_pointer field for variable size non-list types
            if not field.type.fixed_size() and field.c_pointer == ' ':
                field.c_pointer = '*mut '
                field.r_pointer = '&'
            if field.type.is_list and not field.type.member.fixed_size():
                field.c_pointer = '*mut '
                field.r_pointer = '*mut '

            if field.type.is_switch:
                field.c_pointer = '*mut '
                field.r_pointer = '*mut '
                field.c_field_const_type = field.c_field_type
                field.r_field_const_type = field.r_field_type
                self.need_aux = True
            elif not field.type.fixed_size() and not field.type.is_bitcase:
                self.need_sizeof = True

            field.c_iterator_type = _t(field.field_type + ('iterator',))      # xcb_fieldtype_iterator_t
            field.r_iterator_type = _rty(field.field_type + ('iterator',))      # xcb_fieldtype_iterator_t
            field.c_iterator_name = _n(name + (field.field_name, 'iterator')) # xcb_container_field_iterator
            field.c_accessor_name = _n(name + (field.field_name,))            # xcb_container_field
            field.c_length_name = _n(name + (field.field_name, 'length'))     # xcb_container_field_length
            field.c_end_name = _n(name + (field.field_name, 'end'))           # xcb_container_field_end

            field.prev_varsized_field = prev_varsized_field
            field.prev_varsized_offset = prev_varsized_offset

            if prev_varsized_offset == 0:
                first_field_after_varsized = field
            field.first_field_after_varsized = first_field_after_varsized

            if field.type.fixed_size():
                prev_varsized_offset += field.type.size
                # special case: intermixed fixed and variable size fields
                if prev_varsized_field is not None and not field.type.is_pad and field.wire:
                    if not self.is_union:
                        self.need_serialize = True
                        self.var_followed_by_fixed_fields = True
            else:
                self.last_varsized_field = field
                prev_varsized_field = field
                prev_varsized_offset = 0                    

            if self.var_followed_by_fixed_fields:
                if field.type.fixed_size():
                    field.prev_varsized_field = None
                            
    if self.need_serialize:
        # when _unserialize() is wanted, create _sizeof() as well for consistency reasons 
        self.need_sizeof = True

    # as switch does never appear at toplevel, 
    # continue here with type construction
    if self.is_switch:
        if self.c_type not in finished_switch:
            finished_switch.append(self.c_type)
            # special: switch C structs get pointer fields for variable-sized members
            _c_complex(self)
            for bitcase in self.bitcases:
                bitcase_name = bitcase.type.name if bitcase.type.has_name else name
                _c_accessors(bitcase.type, bitcase_name, bitcase_name)
                # no list with switch as element, so no call to 
                # _c_iterator(field.type, field_name) necessary

    if not self.is_bitcase:
        if self.need_serialize:
            if self.c_serialize_name not in finished_serializers:
                finished_serializers.append(self.c_serialize_name)
                _c_serialize('serialize', self)

                # _unpack() and _unserialize() are only needed for special cases:
                #   switch -> unpack
                #   special cases -> unserialize
                if self.is_switch or self.var_followed_by_fixed_fields:
                    _c_serialize('unserialize', self)
                    
        if self.need_sizeof:
            if self.c_sizeof_name not in finished_sizeof:
                if not module.namespace.is_ext or self.name[:2] == module.namespace.prefix:
                    finished_sizeof.append(self.c_sizeof_name)
                    _c_serialize('sizeof', self)
# _c_type_setup()

def _c_helper_absolute_name(prefix, field=None):
    """
    turn prefix, which is a list of tuples (name, separator, Type obj) into a string
    representing a valid name in C (based on the context)
    if field is not None, append the field name as well
    """
    prefix_str = ''
    for name, sep, obj in prefix:
        prefix_str += name
        if '' == sep:
            sep = '->'
            if ((obj.is_bitcase and obj.has_name) or     # named bitcase
                (obj.is_switch and len(obj.parents)>1)):
                sep = '.'
        prefix_str += sep
    if field is not None:
        prefix_str += _cpp(field.field_name)
    return prefix_str
# _c_absolute_name
    
def _c_helper_field_mapping(complex_type, prefix, flat=False):
    """
    generate absolute names, based on prefix, for all fields starting from complex_type
    if flat == True, nested complex types are not taken into account
    """
    all_fields = {}
    if complex_type.is_switch:
        for b in complex_type.bitcases:
            if b.type.has_name:
                switch_name, switch_sep, switch_type = prefix[-1]
                bitcase_prefix = prefix + [(b.type.name[-1], '.', b.type)]
            else:
                bitcase_prefix = prefix 

            if (True==flat and not b.type.has_name) or False==flat:
                all_fields.update(_c_helper_field_mapping(b.type, bitcase_prefix, flat))
    else:
        for f in complex_type.fields:
            fname = _c_helper_absolute_name(prefix, f)
            if f.field_name in all_fields:
                raise Exception("field name %s has been registered before" % f.field_name)

            all_fields[f.field_name] = (fname, f)
            if f.type.is_container and flat==False:
                if f.type.is_bitcase and not f.type.has_name:
                    new_prefix = prefix
                elif f.type.is_switch and len(f.type.parents)>1:
                    # nested switch gets another separator
                    new_prefix = prefix+[(f.c_field_name, '.', f.type)]
                else:
                    new_prefix = prefix+[(f.c_field_name, '->', f.type)]
                all_fields.update(_c_helper_field_mapping(f.type, new_prefix, flat))

    return all_fields
# _c_field_mapping()

def _c_helper_resolve_field_names (prefix):
    """
    get field names for all objects in the prefix array
    """
    all_fields = {}
    tmp_prefix = []
    # look for fields in the remaining containers
    for idx, p in enumerate(prefix):
        name, sep, obj = p
        if ''==sep:
            # sep can be preset in prefix, if not, make a sensible guess
            sep = '.' if (obj.is_switch or obj.is_bitcase) else '->'
            # exception: 'toplevel' object (switch as well!) always have sep '->'
            sep = '->' if idx<1 else sep
        if not obj.is_bitcase or (obj.is_bitcase and obj.has_name):
            tmp_prefix.append((name, sep, obj))
        all_fields.update(_c_helper_field_mapping(obj, tmp_prefix, flat=True))

    return all_fields
# _c_helper_resolve_field_names

def get_expr_fields(self):
    """
    get the Fields referenced by switch or list expression 
    """
    def get_expr_field_names(expr):
        if expr.op is None:
            if expr.lenfield_name is not None:
                return [expr.lenfield_name]
            else:
                # constant value expr
                return []
        else:
            if expr.op == '~':
                return get_expr_field_names(expr.rhs)
            elif expr.op == 'popcount':
                return get_expr_field_names(expr.rhs)
            elif expr.op == 'sumof':
                # sumof expr references another list, 
                # we need that list's length field here
                field = None
                for f in expr.lenfield_parent.fields:
                    if f.field_name == expr.lenfield_name:
                        field = f
                        break
                if field is None:
                    raise Exception("list field '%s' referenced by sumof not found" % expr.lenfield_name)
                # referenced list + its length field
                return [expr.lenfield_name] + get_expr_field_names(field.type.expr)
            elif expr.op == 'enumref':
                return []
            else:
                return get_expr_field_names(expr.lhs) + get_expr_field_names(expr.rhs)
    # get_expr_field_names()
    
    # resolve the field names with the parent structure(s)
    unresolved_fields_names = get_expr_field_names(self.expr)

    # construct prefix from self
    prefix = [('', '', p) for p in self.parents]
    if self.is_container:
        prefix.append(('', '', self))

    all_fields = _c_helper_resolve_field_names (prefix)
    resolved_fields_names = list(filter(lambda x: x in all_fields.keys(), unresolved_fields_names))
    if len(unresolved_fields_names) != len(resolved_fields_names):
        raise Exception("could not resolve all fields for %s" % self.name)
    
    resolved_fields = [all_fields[n][1] for n in resolved_fields_names]
    return resolved_fields
# get_expr_fields()

def resolve_expr_fields(complex_obj):
    """
    find expr fields appearing in complex_obj and descendents that cannot be resolved within complex_obj
    these are normally fields that need to be given as function parameters
    """
    all_fields = []
    expr_fields = []
    unresolved = []

    for field in complex_obj.fields:
        all_fields.append(field)
        if field.type.is_switch or field.type.is_list:
            expr_fields += get_expr_fields(field.type)
        if field.type.is_container:
            expr_fields += resolve_expr_fields(field.type)

    # try to resolve expr fields
    for e in expr_fields:
        if e not in all_fields and e not in unresolved:
            unresolved.append(e)
    return unresolved
# resolve_expr_fields()
            
def get_serialize_params(context, self, buffer_var='_buffer', aux_var='_aux'):
    """
    functions like _serialize(), _unserialize(), and _unpack() sometimes need additional parameters:
    E.g. in order to unpack switch, extra parameters might be needed to evaluate the switch 
    expression. This function tries to resolve all fields within a structure, and returns the 
    unresolved fields as the list of external parameters. 
    """
    def add_param(params, param):
        if param not in params:
            params.append(param)

    # collect all fields into param_fields
    param_fields = []
    wire_fields = []

    for field in self.fields:
        if field.visible:
            # the field should appear as a parameter in the function call
            param_fields.append(field)
        if field.wire and not field.auto:
            if field.type.fixed_size() and not self.is_switch:
                # field in the xcb_out structure
                wire_fields.append(field)
        # fields like 'pad0' are skipped!
               
    # in case of switch, parameters always contain any fields referenced in the switch expr
    # we do not need any variable size fields here, as the switch data type contains both 
    # fixed and variable size fields
    if self.is_switch:
        param_fields = get_expr_fields(self)

    # _serialize()/_unserialize()/_unpack() function parameters
    # note: don't use set() for params, it is unsorted
    params = []
    
    # 1. the parameter for the void * buffer
    if  'serialize' == context:
        params.append(('c_void', '*mut *mut ', buffer_var))
    elif context in ('unserialize', 'unpack', 'sizeof'):
        params.append(('c_void', '*mut ', buffer_var))

    # 2. any expr fields that cannot be resolved within self and descendants
    unresolved_fields = resolve_expr_fields(self)
    for f in unresolved_fields:
        add_param(params, (f.c_field_type, '', f.c_field_name))

    # 3. param_fields contain the fields necessary to evaluate the switch expr or any other fields
    #    that do not appear in the data type struct
    for p in param_fields:
        if self.is_switch:
            typespec = p.c_field_const_type
            pointerspec = p.c_pointer 
            add_param(params, (typespec, pointerspec, p.c_field_name))
        else:
            if p.visible and not p.wire and not p.auto:
                typespec = p.c_field_type
                pointerspec = ''
                add_param(params, (typespec, pointerspec, p.c_field_name))
  
    # 4. aux argument
    if 'serialize' == context:
        add_param(params, ('%s' % self.c_type, '*mut ', aux_var))
    elif 'unserialize' == context: 
        add_param(params, ('%s' % self.c_type, '*mut *mut ', aux_var))
    elif 'unpack' == context:
        add_param(params, ('%s' % self.c_type, '*mut ', aux_var))

    # 5. switch contains all variable size fields as struct members
    #    for other data types though, these have to be supplied separately
    #    this is important for the special case of intermixed fixed and 
    #    variable size fields
    if not self.is_switch and 'serialize' == context:
        for p in param_fields:
            if not p.type.fixed_size():
                add_param(params, (p.c_field_const_type, '*mut ', p.c_field_name))

    return (param_fields, wire_fields, params)
# get_serialize_params()

def _c_serialize(context, self):
    """
    depending on the context variable, generate _serialize(), _unserialize(), _unpack(), or _sizeof() 
    for the ComplexType variable self
    """
    _h_setlevel(1)
    _r_setlevel(1)

    _h('')
    # _serialize() returns the buffer size

    if self.is_switch and 'unserialize' == context:
        context = 'unpack'

    cases = { 'serialize'   : self.c_serialize_name, 
              'unserialize' : self.c_unserialize_name, 
              'unpack'      : self.c_unpack_name, 
              'sizeof'      : self.c_sizeof_name }
    func_name = cases[context]
            
    param_fields, wire_fields, params = get_serialize_params(context, self)
    variable_size_fields = 0
    # maximum space required for type definition of function arguments
    maxtypelen = 0

    # determine N(variable_fields) 
    for field in param_fields:
        # if self.is_switch, treat all fields as if they are variable sized
        if not field.type.fixed_size() or self.is_switch:
            variable_size_fields += 1
    # determine maxtypelen
    for p in params:
        maxtypelen = max(maxtypelen, len(p[0]) + len(p[1]))    

    # write to .c/.h
    indent = ' '*(len(func_name)+2)
    param_str = []
    for p in params:
        typespec, pointerspec, field_name = p
        spacing = ' '*(maxtypelen-len(field_name)-len(pointerspec))
        param_str.append("%s%s :%s  %s%s" % (indent, field_name, spacing, pointerspec, typespec))
    # insert function name
    param_str[0] = "pub fn %s (%s" % (func_name, param_str[0].strip())
    param_str = list(map(lambda x: "%s," % x, param_str))
    for s in param_str[:-1]:
        _h(s)
    _h("%s) -> c_int;" % param_str[-1].rstrip(','))

# _c_serialize()

def _c_iterator_get_end(field, accum):
    '''
    Figures out what C code is needed to find the end of a variable-length structure field.
    For nested structures, recurses into its last variable-sized field.
    For lists, calls the end function
    '''
    if field.type.is_container:
        accum = field.c_accessor_name + '(' + accum + ')'
        return _c_iterator_get_end(field.type.last_varsized_field, accum)
    if field.type.is_list:
        # XXX we can always use the first way
        if field.type.member.is_simple:
            return field.c_end_name + '(' + accum + ')'
        else:
            return field.type.member.c_end_name + '(' + field.c_iterator_name + '(' + accum + '))'

def _c_iterator(self, name):
    '''
    Declares the iterator structure and next/end functions for a given type.
    '''
    _h_setlevel(0)
    _r_setlevel(0)
    _h('/**')
    _h(' * @brief %s', self.c_iterator_type)
    _h(' **/')
    _h('pub struct %s {', self.c_iterator_type)
    _h('    pub data : *mut %s,', self.c_type)
    _h('    pub rem  : c_int,')
    _h('    pub index: c_int')
    _h('}\n')

    _r('pub type %s = %s;\n', self.r_iterator_type, self.c_iterator_type)

    _h_setlevel(1)
    _r_setlevel(1)
    _h('')
    _h('/**')
    _h(' * Get the next element of the iterator')
    _h(' * @param i Pointer to a %s', self.c_iterator_type)
    _h(' *')
    _h(' * Get the next element in the iterator. The member rem is')
    _h(' * decreased by one. The member data points to the next')
    _h(' * element. The member index is increased by sizeof(%s)', self.c_type)
    _h(' *')
    _h(' *')
    _h(' */');
    _h('pub fn %s (i:*mut %s) -> c_void;', self.c_next_name, self.c_iterator_type)

    _h('')
    _h('/**')
    _h(' * Return the iterator pointing to the last element')
    _h(' * @param i An %s', self.c_iterator_type)
    _h(' * @return  The iterator pointing to the last element')
    _h(' *')
    _h(' * Set the current element in the iterator to the last element.')
    _h(' * The member rem is set to 0. The member data points to the')
    _h(' * last element.')
    _h(' */')
    _h('pub fn %s (i:%s) -> ffi::base::generic_iterator;', self.c_end_name, self.c_iterator_type)

    _r('')
    _r('impl<\'s, %s> Iterator<&\'s %s> for %s {', self.r_type, self.r_type, self.r_iterator_type)
    _r('    fn next(&mut self) -> Option<&\'s %s> {', self.r_type)
    _r('        if self.rem == 0 { return None; }')
    _r('        unsafe {')
    _r('            let iter : *mut %s = mem::transmute(self);', self.c_iterator_type)
    _r('            let data = (*iter).data;')
    _r('            %s(iter);', self.c_next_name)
    _r('            Some(mem::transmute(data))')
    _r('        }')
    _r('    }')
    _r('}\n')

def type_pad_type(type):
    if type == 'void':
        return 'u8'
    return type

def _c_accessors_field(self, field):
    '''
    Declares the accessor functions for a non-list field that follows a variable-length field.
    '''
    c_type = self.c_type

    # special case: switch
    switch_obj = self if self.is_switch else None
    if self.is_bitcase:
        switch_obj = self.parents[-1]
    if switch_obj is not None:
        c_type = switch_obj.c_type

    if c_type == 'void':
        c_type = 'c_void'

    ftype = field.c_field_type if field.c_field_type != 'void' else 'c_void'

    _h_setlevel(1)

    if field.type.is_simple:
        _h('')
        _h('/**')
        _h(' * ')
        _h(' * %s : %s', field.c_accessor_name, field.c_field_type,)
        _h(' * ')
        _h(' *')
        _h(' **/')
        _h('pub fn %s (R : *mut %s) -> %s;', field.c_accessor_name, c_type, ftype)
    else:
        _h('')
        _h('')
        _h('/**')
        _h(' *')
        _h(' * %s : *mut %s', field.c_accessor_name, field.c_field_type)
        _h(' * ')
        _h(' *')
        _h(' */')
        if field.type.is_switch and switch_obj is None:
            return_type = '*mut c_void'
        else:
            return_type = '*mut %s' % ftype

        _h('pub fn %s (R : *mut %s) -> %s;', field.c_accessor_name, c_type, return_type)


def _c_accessors_list(self, field):
    '''
    Declares the accessor functions for a list field.
    Declares a direct-accessor function only if the list members are fixed size.
    Declares length and get-iterator functions always.
    '''
    list = field.type
    c_type = self.c_type

    # special case: switch
    # in case of switch, 2 params have to be supplied to certain accessor functions:
    #   1. the anchestor object (request or reply)
    #   2. the (anchestor) switch object
    # the reason is that switch is either a child of a request/reply or nested in another switch, 
    # so whenever we need to access a length field, we might need to refer to some anchestor type
    switch_obj = self if self.is_switch else None
    if self.is_bitcase:
        switch_obj = self.parents[-1]
    if switch_obj is not None:
        c_type = switch_obj.c_type

    params = []
    fields = {}
    parents = self.parents if hasattr(self, 'parents') else [self]
    # 'R': parents[0] is always the 'toplevel' container type 
    params.append(('R : *mut %s' % parents[0].c_type, parents[0]))
    fields.update(_c_helper_field_mapping(parents[0], [('R', '->', parents[0])], flat=True))
    # auxiliary object for 'R' parameters
    R_obj = parents[0]

    if switch_obj is not None:
        # now look where the fields are defined that are needed to evaluate 
        # the switch expr, and store the parent objects in accessor_params and
        # the fields in switch_fields

        # 'S': name for the 'toplevel' switch
        toplevel_switch = parents[1]
        params.append(('S : *mut %s' % toplevel_switch.c_type, toplevel_switch))
        fields.update(_c_helper_field_mapping(toplevel_switch, [('S', '->', toplevel_switch)], flat=True))

        # initialize prefix for everything "below" S
        prefix_str = '/* %s */ S' % toplevel_switch.name[-1]
        prefix = [(prefix_str, '->', toplevel_switch)]

        # look for fields in the remaining containers
        for p in parents[2:] + [self]:
            # the separator between parent and child is always '.' here, 
            # because of nested switch statements
            if not p.is_bitcase or (p.is_bitcase and p.has_name):
                prefix.append((p.name[-1], '.', p))
            fields.update(_c_helper_field_mapping(p, prefix, flat=True))

        # auxiliary object for 'S' parameter
        S_obj = parents[1]

    field.c_field_type = 'c_void' if field.c_field_type == 'void' else field.c_field_type
    _h_setlevel(1)
    _r_setlevel(1)
    if list.member.fixed_size():
        idx = 1 if switch_obj is not None else 0
        _h('')
        _h('pub fn %s (%s) -> *mut %s;', field.c_accessor_name, params[idx][0], field.c_field_type)

    _h('')
    _h('')
    if switch_obj is not None:
        _hr('pub fn %s (R : *mut %s,', field.c_length_name, R_obj.c_type)
        spacing = ' '*(len(field.c_length_name)+7)
        _h('%sS : *mut %s) -> c_int;', spacing, S_obj.c_type)
    else:
        _h('pub fn %s (R : *mut %s) -> c_int;', field.c_length_name, c_type)

    if field.type.member.is_simple:
        _h('')
        _h('')
        if switch_obj is not None:
            _h('pub fn %s (R : %s,', field.c_end_name, R_obj.c_type)
            spacing = ' '*(len(field.c_end_name)+2)
            _h('%sS : *mut %s ) -> ffi::base::generic_iterator;', spacing, S_obj.c_type)
        else:
            _h('pub fn %s (R : *mut %s) -> ffi::base::generic_iterator;', field.c_end_name, c_type)

    else:
        _h('')

        if switch_obj is not None:
            _h('pub fn %s (R : %s,', field.c_iterator_name, R_obj.c_type)
            spacing = ' '*(len(field.c_iterator_name)+2)
            _h('%sS : *mut %s /**< */) -> %s;', spacing, S_obj.c_type, field.c_iterator_type)
        else:
            _h('pub fn %s (R : *mut %s) -> %s;', field.c_iterator_name, c_type, field.c_iterator_type)

def _c_accessors(self, name, base):
    '''
    Declares the accessor functions for the fields of a structure.
    '''
    # no accessors for switch itself - 
    # switch always needs to be unpacked explicitly
#    if self.is_switch:
#        pass
#    else:
    if True:
        for field in self.fields:
            if field.type.is_list and not field.type.fixed_size():
                _c_accessors_list(self, field)
            elif field.prev_varsized_field is not None or not field.type.fixed_size():
                _c_accessors_field(self, field)

    accessor_fields = []
    for f in self.fields:
        if not f.visible:
            continue
        fty = f.type
        accessor_fields.append(f)
        if fty.is_list or fty.is_switch or fty.is_bitcase:
            try:
                accessor_fields.remove(fty.expr.lenfield)
            except: #NB: This sohuld check for a more specific exceptio
                pass

    _r_setlevel(1)
    _r('\nimpl base::%s<%s> {', self.wrap_type, self.c_type)
    for field in accessor_fields:
        _r_accessor(self,field)
    _r_setlevel(1)
    _r('}')


def _r_accessor(self,field):
    _r_setlevel(1)
    if field.type.is_simple:
        _r('  pub fn %s(&self) -> %s {', field.c_field_name, field.r_field_type)
        _r('    unsafe { accessor!(%s -> %s, %s) }', field.c_field_name, field.r_field_type,
                                            self.wrap_field_name)
        _r('  }\n')
    elif field.type.is_list and not field.type.fixed_size():
        if field.type.member.is_simple:
            fty = field.type.member.r_type
            if fty == 'c_char':
                rty = 'String'
                fty = 'str'
            else:
                rty = 'Vec<'+fty+'>'

            _r('  pub fn %s(&self) -> %s {', field.c_field_name, rty)
            _r('    unsafe { accessor!(%s, %s, %s, %s) }', fty, field.c_length_name, field.c_accessor_name,
                                            self.wrap_field_name)
        else:
            _r('  pub fn %s(&self) -> %s {', field.c_field_name, field.r_iterator_type)
            _r('    unsafe { accessor!(%s, %s, %s) }', field.r_iterator_type, field.c_iterator_name,
                                            self.wrap_field_name)
        _r('  }\n')
    elif field.type.is_list:
        _r('  pub fn %s(&self) -> Vec<%s> {', field.c_field_name, field.r_field_type) 
        _r('    unsafe { (%s.%s).to_owned() }',self.wrap_field_name,field.c_field_name)
        _r('  }\n')

    elif field.type.is_container:
        _r('  pub fn %s(&self) -> %s {', field.c_field_name, field.r_field_type)
        _r('    unsafe { mem::transmute(%s.%s) }', self.wrap_field_name, field.c_field_name)
        _r('  }')
        pass;
    else:
        _r('sparklemonkey %s', field.field_name)

def c_simple(self, name):
    '''
    Exported function that handles cardinal type declarations.
    These are types which are typedef'd to one of the CARDx's, char, float, etc.
    '''
    _c_type_setup(self, name, ())

    if (self.name != name):
        # Typedef
        _h_setlevel(0)
        my_name = _t(name)
        _h('')
        _h('pub type %s = %s;', my_name, _t(self.name))
        _r('pub type %s = %s;\n', _rty(name), my_name)

        # Iterator
        _c_iterator(self, name)

def _c_complex(self):
    '''
    Helper function for handling all structure types.
    Called for all structs, requests, replies, events, errors.
    '''

    _h_setlevel(0)
    _h('')
    _h('pub struct %s {', self.c_type)


    struct_fields = []
    maxtypelen = 0

    varfield = None
    for field in self.fields:
        if not field.type.fixed_size() and not self.is_switch and not self.is_union:
            varfield = field.c_field_name
            continue
        if field.wire:
            struct_fields.append(field)
    
    for field in struct_fields:
        length = len(field.c_field_name)
        # account for '*mut ' pointer_spec
        maxtypelen = max(maxtypelen, length)

    def _c_complex_field(self, field, space='', comma=','):
        if (field.type.fixed_size() or 
            # in case of switch with switch children, don't make the field a pointer
            # necessary for unserialize to work
            (self.is_switch and field.type.is_switch)):
            spacing = ' ' * (maxtypelen - len(field.c_field_name))
            if field.c_subscript == 1:
                ftype = field.c_field_type
            else:
                ftype = "[%s,..%d]" % (field.c_field_type, field.c_subscript)
            _h(' %s    pub %s : %s  %s%s', space, field.c_field_name, spacing, ftype, comma)
        else:
            ftype = field.c_field_type
            spacing = ' ' * (maxtypelen - (len(field.c_field_type) + 1))
            _h('%s    pub %s : %s  *mut %s%s', space, field.c_field_name, spacing, ftype, comma)

    if not self.is_switch:
        count = len(struct_fields)
        for field in struct_fields:
            count = count - 1
            _c_complex_field(self, field, '', ',' if count > 0 else '')
    else:
        count = len(self.bitcases)
        for b in self.bitcases:
            count = count - 1
            space = ''
            if b.type.has_name:
                _h('    %s : struct _%s {', b.c_field_name, b.c_field_name)
                space = '    '
                oldcount = count
                count = len(b.type.fields) - 1

            comma = ',' if count > 0 else ''
            for field in b.type.fields:
                _c_complex_field(self, field, space, comma)
            if b.type.has_name:
                count = oldcount
                _h('    }')

    _h('}\n')

def c_struct(self, name):
    '''
    Exported function that handles structure declarations.
    '''
    _c_type_setup(self, name, ())

    _c_complex(self)

    self.wrap_type = 'Struct'
    _r('pub type %s = base::Struct<%s>;\n', self.r_type, self.c_type)
    self.wrap_field_name = 'self.strct'
    _c_accessors(self, name, name)
    _c_iterator(self, name)

def c_union(self, name):
    '''
    Exported function that handles union declarations.
    '''
    _c_type_setup(self, name, ())
    #_c_complex(self)

    field_size = 1
    for field in self.fields:
        if field.type.size and field.type.nmemb:
            field_size = max(field_size, field.type.size*field.type.nmemb)

    _h_setlevel(0)
    _h('')
    _h('pub struct %s {', self.c_type)
    _h('    data : [u8,..%d]', field_size)
    _h('}')

    self.wrap_type = 'Struct'
    _r('pub type %s = base::Struct<%s>;', self.r_type, self.c_type)

    _c_iterator(self, name)

def _c_request_helper(self, name, rust_cookie_type, cookie_type, void, regular, aux=False):
    '''
    Declares a request function.
    '''

    # Four stunningly confusing possibilities here:
    #
    #   Void            Non-void
    # ------------------------------
    # "req"            "req"
    # 0 flag           CHECKED flag   Normal Mode
    # void_cookie      req_cookie
    # ------------------------------
    # "req_checked"    "req_unchecked"
    # CHECKED flag     0 flag         Abnormal Mode
    # void_cookie      req_cookie
    # ------------------------------


    # Whether we are _checked or _unchecked
    checked = void and not regular
    unchecked = not void and not regular

    # What kind of cookie we return
    func_cookie = 'void_cookie' if void else self.c_cookie_type

    # What flag is passed to xcb_request
    func_flags = '0' if (void and regular) or (not void and not regular) else 'XCB_REQUEST_CHECKED'

    # Global extension id variable or NULL for xproto
    func_ext_global = '&' + _ns.c_ext_global_name if _ns.is_ext else '0'

    # What our function name is
    func_name = self.c_request_name if not aux else self.c_aux_name
    rust_func_name = self.r_request_name if not aux else self.r_aux_name
    if checked:
        func_name = self.c_checked_name if not aux else self.c_aux_checked_name
        rust_func_name = self.r_checked_name if not aux else self.r_aux_checked_name
    if unchecked:
        func_name = self.c_unchecked_name if not aux else self.c_aux_unchecked_name
        rust_func_name = self.r_unchecked_name if not aux else self.r_aux_unchecked_name

    param_fields = []
    wire_fields = []
    serial_fields = []
    # special case: list with variable size elements
    list_with_var_size_elems = False

    for field in self.fields:
        if field.visible:
            # The field should appear as a call parameter
            param_fields.append(field)
        if field.wire and not field.auto:
            # We need to set the field up in the structure
            wire_fields.append(field)
        if field.type.need_serialize or field.type.need_sizeof:
            serial_fields.append(field)
        
    for field in param_fields:
        c_field_const_type = field.c_field_const_type 
        if field.type.need_serialize and not aux:
            c_field_const_type = "()"
        if field.type.is_list and not field.type.member.fixed_size():
            list_with_var_size_elems = True

    _h_setlevel(1)
    _r_setlevel(1)
    _h('')
    _h('/**')
    if hasattr(self, "doc") and self.doc:
        if self.doc.brief:
            _h(' * ' + self.doc.brief)
    _h(' *')
    _h(' * @param c The connection')
    param_names = [f.c_field_name for f in param_fields]
    if hasattr(self, "doc") and self.doc:
        for field in param_fields:
            # XXX: hard-coded until we fix xproto.xml
            base_func_name = self.c_request_name if not aux else self.c_aux_name
            if base_func_name == 'change_gc' and field.c_field_name == 'value_mask':
                field.enum = 'GC'
            elif base_func_name == 'change_window_attributes' and field.c_field_name == 'value_mask':
                field.enum = 'CW'
            elif base_func_name == 'create_window' and field.c_field_name == 'value_mask':
                field.enum = 'CW'
            if field.enum:
                # XXX: why the 'xcb' prefix?
                key = ('xcb', field.enum)

                tname = _t(key)
                if namecount[tname] > 1:
                    tname = _t(key + ('enum',))
                _h(' * @param %s A bitmask of #%s values.' % (field.c_field_name, tname))

            if self.doc and field.field_name in self.doc.fields:
                desc = self.doc.fields[field.field_name]
                for name in param_names:
                    desc = desc.replace('`%s`' % name, '\\a %s' % (name))
                desc = desc.split("\n")
                desc = [line if line != '' else '\\n' for line in desc]
                _h(' * @param %s %s' % (field.c_field_name, "\n * ".join(desc)))
            # If there is no documentation yet, we simply don't generate an
            # @param tag. Doxygen will then warn about missing documentation.

    _h(' * @return A cookie')
    _h(' *')
    if hasattr(self, "doc") and self.doc:
        if self.doc.description:
            desc = self.doc.description
            for name in param_names:
                desc = desc.replace('`%s`' % name, '\\a %s' % (name))
            desc = desc.split("\n")
            _h(' * ' + "\n * ".join(desc))
    else:
        _h(' * Delivers a request to the X server.')
    _h(' * ')
    if checked:
        _h(' * This form can be used only if the request will not cause')
        _h(' * a reply to be generated. Any returned error will be')
        _h(' * saved for handling by xcb_request_check().')
    if unchecked:
        _h(' * This form can be used only if the request will cause')
        _h(' * a reply to be generated. Any returned error will be')
        _h(' * placed in the event queue.')
    _h(' */')
    count = len(param_fields)
    comma = ',' if count else (') -> %s;' % (cookie_type,))
    _h('pub fn %s (c : *mut ffi::base::connection%s', func_name, comma)

    func_spacing = ' ' * (len(func_name) + 12)
    for field in param_fields:
        count = count - 1
        c_field_const_type = field.c_field_const_type
        if c_field_const_type == 'void':
            c_field_const_type = 'c_void'
        c_pointer = field.c_pointer
        if field.type.need_serialize and not aux:
            c_field_const_type = "()"
            c_pointer = '*mut '
        comma = ',' if count else (') -> %s;' % (cookie_type,))
        _h('%s%s : %s%s%s', func_spacing, field.c_field_name,
                c_pointer, c_field_const_type, comma)

    for f in param_fields:
        f.skip = False

    idx = 0
    r_fields = []
    for f in param_fields:
        f.idx = idx
        idx = idx + 1
        f.skip = False
        if not f.visible or f.auto:
            continue
        fty = f.type
        if fty.is_list or fty.is_switch or fty.is_bitcase:
            len_name = f.c_field_name+'_len'
            f.lf = None
            i = 0
            for fl in param_fields:
                if fl.skip:
                    continue
                if f.type.expr.lenfield == fl:
                    fl.skip=True
                    f.lf = fl
                    f.lfidx = i
                    break
                elif fl.c_field_name == len_name:
                    fl.skip=True
                    f.lf = fl
                    f.lfidx = i
                    break
                i = i + 1
        f.idx = idx
        r_fields.append(f)

    call_params = []
    mk_params = []

    func_spacing = ' ' * (len(rust_func_name) + 9)
    count = len(r_fields)
    comma = ',' if count else (') -> %s<\'r> {' % (rust_cookie_type,))
    _r('pub fn %s<\'r> (c : &\'r Connection%s', rust_func_name, comma)
    for field in r_fields:
        count = count - 1
        if field.skip or field.auto:
            continue
        fty = field.type

        field_type = field.r_field_type
        c_field_type = field.c_field_type

        c_field_const_type = field.c_field_const_type
        if c_field_const_type == 'void':
            c_field_const_type = 'c_void'
        c_pointer = field.c_pointer
        if field.type.need_serialize and not aux:
            c_field_const_type = "()"
            c_pointer = '*mut '

        if fty.is_list:
            if fty.expr.bitfield:
                mk_params.append("let (%s_mask, %s_vec) = pack_bitfield(%s);" %
                        (field.c_field_name, field.c_field_name, field.c_field_name))
                mk_params.append("let %s_ptr = %s_vec.as_mut_ptr();" %
                        (field.c_field_name, field.c_field_name))

                call_params.append((field.idx-1,'%s_mask as %s' %(field.c_field_name,
                    fty.expr.lenfield.c_field_type)))
                call_params.append((field.idx, '%s_ptr as *mut %s' % (field.c_field_name,c_field_const_type)))

                field_type = '&[(%s,%s)]' % (fty.expr.lenfield.r_field_type, field_type)
            else:
                if fty.member.r_type == 'c_char':
                    field_type = '&str'
                    mk_params.append("let %s = (%s).as_bytes();" % (field.c_field_name,
                        field.c_field_name))
                elif fty.member.r_type == 'c_void':
                    field_type = '&[u8]'
                else:
                    field_type = '&[%s]' % field_type


                if field.lf:
                    lfty = field.lf.c_field_const_type
                    mk_params.append("let %s_len = %s.len();" % (field.c_field_name, field.c_field_name))
                    call_params.append((field.lf.idx, '%s_len as %s' % (field.c_field_name,lfty)))

                mk_params.append("let %s_ptr = %s.as_mut_ptr();" % (field.c_field_name,
                    field.c_field_name))
                call_params.append((field.idx, '%s_ptr as *mut %s' % (field.c_field_name,
                    c_field_const_type)))


        elif fty.is_container:
            call_params.append((field.idx, '%s.strct' % (field.c_field_name,)))
        else:
            call_params.append((field.idx, '%s as %s' % (field.c_field_name, c_field_const_type)))

        comma = ',' if count else (') -> %s<\'r> {' % (rust_cookie_type,))
        _r('%s%s : %s%s', func_spacing, field.c_field_name, field_type, comma)

    _r('  unsafe {')
    for p in mk_params:
        _r('    %s', p)

    count = len(call_params)
    comma = ',' if count else ');'
    _r('    let cookie = %s(c.get_raw_conn()%s', func_name, comma)
    call_params.sort(lambda x,y: cmp(x[0] , y[0]))
    for idx, c in call_params:
        count = count - 1
        comma = ',' if count else ');'
        _r('        %s%s //%d', c, comma, idx)


    _r('    Cookie {cookie:cookie,conn:c,checked:%s}', 'true' if checked else 'false')
    _r('  }')
    _r('}')

def _c_reply(self, name):
    '''
    Declares the function that returns the reply structure.
    '''
    spacing = ' ' * (len(self.c_reply_name))
    
    _h('')
    _h('/**')
    _h(' * Return the reply')
    _h(' * @param c      The connection')
    _h(' * @param cookie The cookie')
    _h(' * @param e      The generic_error supplied')
    _h(' *')
    _h(' * Returns the reply of the request asked by')
    _h(' * ')
    _h(' * The parameter @p e supplied to this function must be NULL if')
    _h(' * %s(). is used.', self.c_unchecked_name)
    _h(' * Otherwise, it stores the error if any.')
    _h(' *')
    _h(' * The returned value must be freed by the caller using free().')
    _h(' */')
    _h('pub fn %s (c : *mut ffi::base::connection,', self.c_reply_name)
    _h('          %s  cookie : %s,', spacing, self.c_cookie_type)
    _h('          %s  e : *mut *mut ffi::base::generic_error) -> *mut %s;', spacing, self.c_reply_type)

    _r('impl_reply_cookie!(%s<\'s>, %s, %s, %s)\n', self.r_cookie_type, self.c_reply_type, self.r_reply_type, self.c_reply_name)

def _c_opcode(name, opcode):
    '''
    Declares the opcode define for requests, events, and errors.
    '''
    _h_setlevel(0)
    _r_setlevel(0)
    _h('')
    _r('/** Opcode for %s. */', _n(name))
    _r('pub static %s : u8 = %s;', _n(name).upper(), opcode)
    
def _c_cookie(self, name):
    '''
    Declares the cookie type for a non-void request.
    '''
    _h_setlevel(0)
    _r_setlevel(0)
    _h('')
    _h('pub struct %s {', self.c_cookie_type)
    _h('    sequence : c_uint')
    _h('}')

    self.wrap_type = 'Cookie'
    _r('pub type %s<\'s> = base::Cookie<\'s, %s>;\n', self.r_cookie_type, self.c_cookie_type)


def c_request(self, name):
    '''
    Exported function that handles request declarations.
    '''
    _c_type_setup(self, name, ('request',))


    if self.reply:
        # Cookie type declaration
        _c_cookie(self, name)

    # Opcode define
    _c_opcode(name, self.opcode)

    # Request structure declaration
    _c_complex(self)
    #_r('pub type %s = base::Request<%s>;', self.r_type, self.c_type)

    if self.reply:
        _c_type_setup(self.reply, name, ('reply',))
        _r('pub type %s = base::Reply<%s>;', self.r_reply_type, self.c_reply_type)


        # Reply structure definition
        _c_complex(self.reply)

        # Request prototypes
        _c_request_helper(self, name, self.r_cookie_type, self.c_cookie_type, False, True)
        _c_request_helper(self, name, self.r_cookie_type, self.c_cookie_type, False, False)
        if self.need_aux:
            _c_request_helper(self, name, self.r_cookie_type, self.c_cookie_type, False, True, True)
            _c_request_helper(self, name, self.r_cookie_type, self.c_cookie_type, False, False, True)
        # Reply accessors
        self.reply.wrap_field_name = '(*self.reply)'

        self.reply.wrap_type = "Reply"
        _c_accessors(self.reply, name + ('reply',), name)
        _c_reply(self, name)
    else:
        # Request prototypes
        _c_request_helper(self, name, 'base::VoidCookie', 'ffi::base::void_cookie', True, False)
        _c_request_helper(self, name, 'base::VoidCookie', 'ffi::base::void_cookie', True, True)
        if self.need_aux:
            _c_request_helper(self, name, 'base::VoidCookie', 'ffi::base::void_cookie', True, False, True)
            _c_request_helper(self, name, 'base::VoidCookie', 'ffi::base::void_cookie', True, True, True)

    cookie_type = self.c_cookie_type if self.reply else 'void_cookie'

def c_event(self, name):
    '''
    Exported function that handles event declarations.
    '''
    _c_type_setup(self, name, ('event',))

    # Opcode define
    _c_opcode(name, self.opcodes[name])

    self.wrap_type = 'Event';
    _r('pub type %s = base::Event<%s>;', self.r_type, self.c_type)

    if self.name == name:
        # Structure definition
        _c_complex(self)

        self.wrap_field_name = '(*self.event)'

        accessor_fields = []
        for f in self.fields:
            if not f.visible:
                continue
            fty = f.type
            accessor_fields.append(f)
            if fty.is_list or fty.is_switch or fty.is_bitcase:
                try:
                    accessor_fields.remove(fty.expr.lenfield)
                except: #NB: This should check for a more specific exceptio
                    pass

        new_params = []

        _r_setlevel(1)
        _r('\nimpl base::%s<%s> {', self.wrap_type, self.c_type)
        for field in accessor_fields:
            _r_accessor(self,field)

            fty = field.type
            ftype = field.r_field_type;
            if fty.is_list:
                ftype = '[%s,..%d]' % (ftype, fty.nmemb)

            new_params.append('%s : %s' % (field.c_field_name, ftype))

        _r('  pub fn new('+(',\n         '.join(new_params))+') -> %s {', self.r_type)
        _r('    unsafe {')
        _r('      let raw = malloc(32u as size_t) as *mut %s;', self.c_type)
        for f in self.fields:
            if not f.visible: continue
            if f.type.is_container:
                _r('      (*raw).%s = %s.strct;', f.c_field_name, f.c_field_name)
            else:
                _r('      (*raw).%s = %s;', f.c_field_name, f.c_field_name)
        _r('      Event { event : raw as *mut %s }', self.c_type)
        _r('    }')
        _r('  }')
        _r('}')

    else:
        # Typedef
        _h('')
        _h('pub type %s = %s;', _t(name + ('event',)), _t(self.name + ('event',)))

def c_error(self, name):
    '''
    Exported function that handles error declarations.
    '''
    _c_type_setup(self, name, ('error',))

    # Opcode define
    _c_opcode(name, self.opcodes[name])

    if self.name == name:
        # Structure definition
        _c_complex(self)
    else:
        # Typedef
        _h('')
        _h('pub type %s  = %s;', _t(name + ('error',)), _t(self.name + ('error',)))

    _r('pub type %s = base::Error<%s>;', self.r_type, self.c_type)



# Main routine starts here

# Must create an "output" dictionary before any xcbgen imports.
output = {'open'    : c_open,
          'close'   : c_close,
          'simple'  : c_simple,
          'enum'    : c_enum,
          'struct'  : c_struct,
          'union'   : c_union,
          'request' : c_request,
          'event'   : c_event,
          'error'   : c_error,
          }

# Boilerplate below this point

# Check for the argument that specifies path to the xcbgen python package.
try:
    opts, args = getopt.getopt(sys.argv[1:], 'o:')
except getopt.GetoptError as err:
    print(err)
    print('Usage: r_client.py [-o outdir] file.xml')
    sys.exit(1)

for (opt, arg) in opts:
    if opt == '-o':
        outdir = arg

# Import the module class
try:
    from xcbgen.state import Module
    from xcbgen.xtypes import *
except ImportError:
    print('''
Failed to load the xcbgen Python package!
Make sure that xcb/proto installed it on your Python path.
If not, you will need to create a .pth file or define $PYTHONPATH
to extend the path.
Refer to the README file in xcb/proto for more info.
''')
    raise

today = time.strftime('%Y-%m-%d', time.gmtime(os.path.getmtime(args[0])))

# Parse the xml header
module = Module(args[0], output)

# Build type-registry and resolve type dependencies
module.register()
module.resolve()

# Output the code
module.generate()

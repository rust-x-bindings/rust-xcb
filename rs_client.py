#!/usr/bin/env python3
'''
script that generates rust code from xcb xml definitions
Each invokation of this script generates one ffi file and one
rust file for an extension or the main X Protocol.

Usage: ./rs_client.py -o src xml/xproto.xml
'''

import sys
import os
import time
import re



class SourceFile(object):
    '''
    buffer to append code in various sections of a file
    in any order
    '''

    _one_indent_level = '    '

    def __init__(self):
        self._section = 0
        self._lines = []
        self._indents = []

    def getsection(self):
        return self._section

    def section(self, section):
        '''
        Set the section of the file where to append code.
        Allows to make different sections in the file to append
        to in any order
        '''
        while len(self._lines) <= section:
            self._lines.append([])
        while len(self._indents) <= section:
            self._indents.append(0)
        self._section = section

    def getindent(self):
        '''
        returns indentation of the current section
        '''
        return self._indents[self._section]

    def setindent(self, indent):
        '''
        sets indentation of the current section
        '''
        self._indents[self._section] = indent;

    def indent(self):
        '''
        adds one level of indentation to the current section
        '''
        self._indents[self._section] += 1

    def unindent(self):
        '''
        removes one level of indentation to the current section
        '''
        assert self.getindent() > 0, "negative indent"
        self._indents[self._section] -= 1

    def __call__(self, fmt, *args):
        '''
        Append a line to the file at in its current section and
        indentation of the current section
        '''
        indent = SourceFile._one_indent_level * self._indents[self._section]
        self._lines[self._section].append(indent + (fmt % args))


    def writeout(self, path):
        with open(path, 'w') as f:
            for section in self._lines:
                for line in section:
                    print(line.rstrip(), file=f)


# FFI source file
_f = SourceFile()

# Rust interface file
_r = SourceFile()

# utility to add same code in both files
def _rf(fmt, *args):
    _f(fmt, *args)
    _r(fmt, *args)


_ns = None
_ext_names = {}

# global variable to keep track of serializers and
# switch data types due to weird dependencies
finished_serializers = []
finished_sizeof = []
finished_switch = []

# exported functions to xcbgen start by 'rs_'

# starting with opening and closing

def rs_open(module):
    '''
    Handles module open.
    module is a xcbgen.state.Module object
    '''
    global _ns
    _ns = module.namespace

    EnumCodegen.build_collision_table(module)

    linklib = "xcb"
    if _ns.is_ext:
        linklib = linklib + '-' + _ns.header
        _ext_names[_ns.ext_name] = _ns.header
        for (n, h) in module.direct_imports:
            _ext_names[n] = h

    _r.section(0)
    _f.section(0)
    _rf('// edited from %s by rs_client.py on %s',
            _ns.file, time.strftime('%c'))
    _rf('// do not edit!')
    _rf('')

    _f('')
    _f('use ffi::base::*;')
    _f('use libc::{c_char, c_int, c_uint, c_void};')

    if _ns.is_ext:
        for (n, h) in module.direct_imports:
            _f('use ffi::%s::*;', h)
        _f('')
        _f('pub const XCB_%s_MAJOR_VERSION: u32 = %s;',
                    _ns.ext_name.upper(),
                    _ns.major_version)
        _f('pub const XCB_%s_MINOR_VERSION: u32 = %s;',
                    _ns.ext_name.upper(),
                    _ns.minor_version)

    _f.section(1)
    _f('')
    _f('')
    _f('#[link(name="%s")]', linklib)
    _f('extern {')
    _f.indent()

    _r('')
    _r('use base;')
    _r('use ffi::%s::*;', _ns.header)
    if _ns.is_ext:
        for (n, h) in module.direct_imports:
            _r('use %s;', h)

    _r.section(1)
    _r('')
    _r('')



def rs_close(module):
    '''
    Handles module close.
    module is a xcbgen.state.Module object.
    main task is to write the files out
    '''

    _f.section(1)

    _f('')
    _f.unindent()
    _f('} // extern')

    _f.writeout(os.path.join(module.rs_srcdir, "ffi", "%s.rs" % _ns.header))
    _r.writeout(os.path.join(module.rs_srcdir, "%s.rs" % _ns.header))



# transformation of name tuples

_cname_re = re.compile('([A-Z0-9][a-z]+|[A-Z0-9]+(?![a-z])|[a-z]+)')
_rs_keywords = ['type', 'str', 'match']


def _tit_split(string):
    '''
    splits string with '_' on each titlecase letter
    >>> _tit_split('SomeString')
    Some_String
    >>> _tit_split('WINDOW')
    WINDOW
    '''
    split = _cname_re.finditer(string)
    name_parts = [match.group(0) for match in split]
    return '_'.join(name_parts)

def _tit_cap(string):
    '''
    capitalize each substring beggining by a titlecase letter
    >>> _tit_cap('SomeString')
    SomeString
    >>> _tit_cap('WINDOW')
    Window
    '''
    split = _cname_re.finditer(string)
    name_parts = [match.group(0) for match in split]
    name_parts = [i[0].upper() + i[1:].lower() for i in name_parts]
    return ''.join(name_parts)

def _symbol(string):
    if string in _rs_keywords:
        string += '_'
    return string

def _upper_1st(string):
    '''
    return copy of string with first letter turned into upper.
    Other letters are untouched.
    '''
    if len(string) == 0:
        return ''
    if len(string) == 1:
        return string.upper()
    return string[0].upper() + string[1:]

def _upper_name(nametup):
    '''
    return a string made from a nametuple with all upper case
    joined with underscore
    >>> _upper_name(('xcb', 'constant', 'AwesomeValue'))
    XCB_CONSTANT_AWESOME_VALUE
    '''
    return '_'.join(tuple(_tit_split(name) for name in nametup)).upper()

def _cap_name(nametup):
    '''
    return a string made from a nametuple with joined title case
    >>> _cap_name(('xcb', 'Type', 'Name'))
    XcbTypeName
    >>> _cap_name(('xcb', 'TypeName'))
    XcbTypeName
    >>> _cap_name(('xcb', 'TYPENAME'))
    XcbTypename
    '''
    return ''.join(tuple(_upper_1st(name) for name in nametup))

def _lower_name(nametup):
    '''
    return a string made from a nametuple with all lower case
    joined with underscore
    >>> _upper_name(('xcb', 'Ext', 'RequestName'))
    xcb_ext_request_name
    '''
    return '_'.join(tuple(_tit_split(name) for name in nametup)).lower()


def _ext_nametup(nametup):
    '''
    return the nametup with 2nd name lowered if module is an extension
    >>> _ext_nametup(('u32',))
    ('u32',)
    >>> _ext_nametup(('xcb', 'XprotoType'))
    ('xcb', 'XprotoType')
    >>> _ext_nametup(('xcb', 'RandR', 'SuperType'))
    ('xcb', 'randr', 'SuperType')
    '''
    if len(nametup) > 2 and _ns.is_ext:
        # lowers extension to avoid '_' split with title letters
        nametup = tuple(name.lower() if i == 1 else name
                for (i, name) in enumerate(nametup))
    return nametup

def _ffi_type_name(nametup):
    '''
    turns the nametup into a FFI type
    >>> _ffi_type_name(('u32',))
    u32
    >>> _ffi_type_name(('xcb', 'XprotoType'))
    xcb_xproto_type_t
    >>> _ffi_type_name(('xcb', 'RandR', 'SuperType'))
    xcb_randr_super_type_t
    '''
    if len(nametup) == 1:
        # handles SimpleType
        return nametup[0]
    return _ffi_name(nametup + ('t',))

def _ffi_name(nametup):
    '''
    turns the nametup into a FFI name
    >>> _ffi_type_name(('u32',))
    u32
    >>> _ffi_type_name(('xcb', 'XprotoType', 't'))
    xcb_xproto_type_t
    >>> _ffi_type_name(('xcb', 'RandR', 'SuperType', 't'))
    xcb_randr_super_type_t
    '''
    return _lower_name(_ext_nametup(nametup))


def _rs_type_name(nametup):
    '''
    turns the nametup into a Rust type name
    foreign rust type names include module prefix
    >>> _rs_type_name(('u32',))
    u32
    >>> _rs_type_name(('xcb', 'Type'))
    xproto::Type
    >>> _rs_type_name(('xcb', 'RandR', 'SuperType'))
    randr::SuperType
    '''
    # handles SimpleType
    if len(nametup) == 1:
        return nametup[0]

    # remove 'xcb'
    if nametup[0].lower() == 'xcb':
        nametup = nametup[1:]

    module = ''
    # handle extension type
    if nametup[0] in _ext_names:
        ext = _ext_names[nametup[0]]
        if (not _ns.is_ext or
                ext != _ns.header):
            module = ext + '::'
        nametup = nametup[1:]

    # handle xproto type
    elif len(nametup) == 1:
        if _ns.is_ext:
            module = 'xproto::'

    return module + ''.join([_tit_cap(n) for n in nametup])



# FFI codegen functions

def _ffi_type_setup(typeobj, nametup, suffix=()):
    '''
    Sets up all the C-related state by adding additional data fields to
    all Field and Type objects.  Here is where we figure out most of our
    variable and function names.

    Recurses into child fields and list member types.
    '''
    # Do all the various names in advance
    typeobj.ffi_type = _ffi_type_name(nametup + suffix)

    typeobj.ffi_iterator_type = _ffi_type_name(nametup + ('iterator',))
    typeobj.ffi_next_fn = _ffi_name(nametup + ('next',))
    typeobj.ffi_end_fn = _ffi_name(nametup + ('end',))

    typeobj.ffi_request_fn = _ffi_name(nametup)
    typeobj.ffi_checked_fn = _ffi_name(nametup + ('checked',))
    typeobj.ffi_unchecked_fn = _ffi_name(nametup + ('unchecked',))
    typeobj.ffi_reply_fn = _ffi_name(nametup + ('reply',))
    typeobj.ffi_reply_type = _ffi_type_name(nametup + ('reply',))
    typeobj.ffi_cookie_type = _ffi_type_name(nametup + ('cookie',))
    typeobj.ffi_reply_fds_fn = _ffi_name(nametup + ('reply_fds',))

    typeobj.ffi_need_aux = False
    typeobj.ffi_need_serialize = False
    typeobj.ffi_need_sizeof = False

    typeobj.ffi_aux_fn = _ffi_name(nametup + ('aux',))
    typeobj.ffi_aux_checked_fn = _ffi_name(nametup + ('aux', 'checked'))
    typeobj.ffi_aux_unchecked_fn = _ffi_name(nametup + ('aux', 'unchecked'))
    typeobj.ffi_serialize_fn = _ffi_name(nametup + ('serialize',))
    typeobj.ffi_unserialize_fn = _ffi_name(nametup + ('unserialize',))
    typeobj.ffi_unpack_fn = _ffi_name(nametup + ('unpack',))
    typeobj.ffi_sizeof_fn = _ffi_name(nametup + ('sizeof',))

    # special case: structs where variable size fields are followed by fixed size fields
    typeobj.ffi_var_followed_by_fixed_fields = False

    if typeobj.is_switch:
        typeobj.ffi_need_serialize = True
        for bitcase in typeobj.bitcases:
            bitcase.ffi_field_name = _symbol(bitcase.field_name)
            bitcase_name = bitcase.field_type if bitcase.type.has_name else nametup
            _ffi_type_setup(bitcase.type, bitcase_name, ())

    elif typeobj.is_container:

        prev_varsized_field = None
        prev_varsized_offset = 0
        first_field_after_varsized = None

        for field in typeobj.fields:
            _ffi_type_setup(field.type, field.field_type, ())
            if field.type.is_list:
                _ffi_type_setup(field.type.member, field.field_type, ())
                if (field.type.nmemb is None):
                    typeobj.ffi_need_sizeof = True

            field.ffi_field_type = _ffi_type_name(field.field_type)
            field.ffi_field_name = _symbol(field.field_name)
            field.has_subscript = (field.type.nmemb and
                            field.type.nmemb > 1)
            field.ffi_need_const = (field.type.nmemb != 1)
            field.ffi_need_pointer = (field.type.nmemb != 1)

            # correct the need_pointer field for variable size non-list types
            if not field.type.fixed_size():
                field.ffi_need_pointer = True
            if field.type.is_list and not field.type.member.fixed_size():
                field.ffi_need_pointer = True

            if field.type.is_switch:
                field.ffi_need_const = True
                field.ffi_need_pointer = True
                field.ffi_need_aux = True
            elif not field.type.fixed_size() and not field.type.is_bitcase:
                typeobj.ffi_need_sizeof = True

            field.ffi_iterator_type = _ffi_type_name(
                    field.field_type + ('iterator',))
            field.ffi_iterator_fn = _ffi_name(
                    nametup + (field.field_name, 'iterator'))
            field.ffi_accessor_fn = _ffi_name(
                    nametup + (field.field_name,))
            field.ffi_length_fn = _ffi_name(
                    nametup + (field.field_name, 'length'))
            field.ffi_end_fn = _ffi_name(
                    nametup + (field.field_name, 'end'))

            field.prev_varsized_field = prev_varsized_field
            field.prev_varsized_offset = prev_varsized_offset

            if prev_varsized_offset == 0:
                first_field_after_varsized = field
            field.first_field_after_varsized = first_field_after_varsized

            if field.type.fixed_size():
                prev_varsized_offset += field.type.size
                # special case: intermixed fixed and variable size fields
                if prev_varsized_field is not None and not field.type.is_pad and field.wire:
                    if not typeobj.is_union:
                        typeobj.ffi_need_serialize = True
                        typeobj.ffi_var_followed_by_fixed_fields = True
            else:
                typeobj.last_varsized_field = field
                prev_varsized_field = field
                prev_varsized_offset = 0

            if typeobj.ffi_var_followed_by_fixed_fields:
                if field.type.fixed_size():
                    field.prev_varsized_field = None

    if typeobj.ffi_need_serialize:
        # when _unserialize() is wanted, create _sizeof() as well for consistency reasons
        typeobj.ffi_need_sizeof = True

    # as switch does never appear at toplevel,
    # continue here with type construction
    if typeobj.is_switch:
        if typeobj.ffi_type not in finished_switch:
            finished_switch.append(typeobj.ffi_type)
            # special: switch C structs get pointer fields for variable-sized members
            _ffi_struct(typeobj)
            for bitcase in typeobj.bitcases:
                bitcase_name = bitcase.type.name if bitcase.type.has_name else nametup
                _ffi_accessors(bitcase.type, bitcase_name)
                # no list with switch as element, so no call to
                # _c_iterator(field.type, field_name) necessary

    if not typeobj.is_bitcase:
        if typeobj.ffi_need_serialize:
            if typeobj.ffi_serialize_fn not in finished_serializers:
                finished_serializers.append(typeobj.ffi_serialize_fn)
                #_ffi_serialize('serialize', typeobj)

                # _unpack() and _unserialize() are only needed for special cases:
                #   switch -> unpack
                #   special cases -> unserialize
                if typeobj.is_switch or typeobj.ffi_var_followed_by_fixed_fields:
                    pass
                    #_ffi_serialize('unserialize', typeobj)

        if typeobj.ffi_need_sizeof:
            if typeobj.ffi_sizeof_fn not in finished_sizeof:
                if not _ns.is_ext or typeobj.name[:2] == _ns.prefix:
                    finished_sizeof.append(typeobj.ffi_sizeof_fn)
                    #_ffi_serialize('sizeof', typeobj)




def _ffi_bitcase_name(switch, bitcase):
    assert switch.is_switch and bitcase.type.has_name
    switch_name = _lower_name(_ext_nametup(switch.name))
    return '_%s__%s' % (switch_name, bitcase.ffi_field_name)


def _ffi_struct(typeobj, must_pack=False):
    '''
    Helper function for handling all structure types.
    Called for structs, requests, replies, events, errors...
    '''

    struct_fields = []

    for field in typeobj.fields:
        if (not field.type.fixed_size()
            and not typeobj.is_switch
            and not typeobj.is_union):
            continue
        if field.wire:
            struct_fields.append(field)

    _f.section(0)
    _f('')
    _f('#[repr(C%s)]', ', packed' if must_pack else '')
    _f('pub struct %s {', typeobj.ffi_type)
    _f.indent()

    maxfieldlen = 0
    if not typeobj.is_switch:
        for field in typeobj.fields:
            maxfieldlen = max(maxfieldlen, len(field.ffi_field_name))
    else:
        for b in typeobj.bitcases:
            if b.type.has_name:
                maxfieldlen = max(maxfieldlen, len(b.ffi_field_name))
            else:
                for field in b.type.fields:
                    maxfieldlen = max(maxfieldlen, len(field.ffi_field_name))



    def _ffi_struct_field(field):
        ftype = field.ffi_field_type
        space = ' '* (maxfieldlen - len(field.ffi_field_name))
        if (field.type.fixed_size() or typeobj.is_union or
            # in case of switch with switch children,
            # don't make the field a pointer
            # necessary for unserialize to work
            (typeobj.is_switch and field.type.is_switch)):
            if field.has_subscript:
                ftype = '[%s; %d]' % (ftype, field.type.nmemb)
            _f('pub %s: %s%s,', field.ffi_field_name, space, ftype)
        else:
            assert not field.has_subscript
            _f('pub %s: %s*mut %s,', field.ffi_field_name, space, ftype)

    named_bitcases = []

    if not typeobj.is_switch:
        for field in struct_fields:
            _ffi_struct_field(field)
    else:
        for b in typeobj.bitcases:
            if b.type.has_name:
                named_bitcases.append(b)
                space = ' ' * (maxfieldlen - len(b.ffi_field_name))
                _f('pub %s: %s%s,', b.ffi_field_name, space,
                        _ffi_bitcase_name(typeobj, b))
            else:
                for field in b.type.fields:
                    _ffi_struct_field(field)

    _f.unindent()
    _f('}')

    for b in named_bitcases:
        _f('')
        _f('#[repr(C)]')
        _f('pub struct %s {', _ffi_bitcase_name(typeobj, b))
        _f.indent()
        maxfieldlen = 0
        for field in b.type.fields:
            maxfieldlen = max(maxfieldlen, len(field.ffi_field_name))
        for field in b.type.fields:
            _ffi_struct_field(field)
        _f.unindent()
        _f('}')



def _ffi_opcode(nametup, opcode):
    _f.section(0)
    _f('')
    _f('pub const %s: u32 = %s;',
            _upper_name(_ext_nametup(nametup)), opcode)


def _ffi_accessors_list(typeobj, field):
    '''
    Declares the accessor functions for a list field.
    Declares a direct-accessor function only if the list members are fixed size.
    Declares length and get-iterator functions always.
    '''

    list = field.type
    ffi_type = typeobj.ffi_type

    # special case: switch
    # in case of switch, 2 params have to be supplied to certain accessor functions:
    #   1. the anchestor object (request or reply)
    #   2. the (anchestor) switch object
    # the reason is that switch is either a child of a request/reply or nested in another switch,
    # so whenever we need to access a length field, we might need to refer to some anchestor type
    switch_obj = typeobj if typeobj.is_switch else None
    if typeobj.is_bitcase:
        switch_obj = typeobj.parents[-1]
    if switch_obj is not None:
        ffi_type = switch_obj.ffi_type

    params = []
    parents = typeobj.parents if hasattr(typeobj, 'parents') else [typeobj]
    # 'R': parents[0] is always the 'toplevel' container type
    params.append(('R: *const %s' % parents[0].ffi_type, parents[0]))
    # auxiliary object for 'R' parameters
    R_obj = parents[0]

    if switch_obj is not None:
        # now look where the fields are defined that are needed to evaluate
        # the switch expr, and store the parent objects in accessor_params and
        # the fields in switch_fields

        # 'S': name for the 'toplevel' switch
        toplevel_switch = parents[1]
        params.append(('S: *const %s' % toplevel_switch.ffi_type, toplevel_switch))

        # auxiliary object for 'S' parameter
        S_obj = parents[1]

    _f.section(1)
    if list.member.fixed_size():
        idx = 1 if switch_obj is not None else 0
        _f('')
        _f('pub fn %s (%s)', field.ffi_accessor_fn, params[idx][0])
        _f('        -> *mut %s;', field.ffi_field_type)

    def _may_switch_fn(fn_name, return_type):
        _f('')
        if switch_obj is not None:
            fn_start = 'pub fn %s (' % fn_name
            spacing = ' '*len(fn_start)
            _f('%sR: *const %s,', fn_start, R_obj.ffi_type)
            _f('%sS: *const %s)', spacing, S_obj.ffi_type)
            _f('        -> %s;', return_type)
        else:
            _f('pub fn %s (R: *const %s)', fn_name, ffi_type)
            _f('        -> %s;', return_type)

    _may_switch_fn(field.ffi_length_fn, 'c_int')

    if field.type.member.is_simple:
        _may_switch_fn(field.ffi_end_fn, 'xcb_generic_iterator_t')
    else:
        _may_switch_fn(field.ffi_iterator_fn, field.ffi_iterator_type)



def _ffi_accessors_field(typeobj, field):
    '''
    Declares the accessor functions for a non-list field that follows a variable-length field.
    '''
    ffi_type = typeobj.ffi_type

    # special case: switch
    switch_obj = typeobj if typeobj.is_switch else None
    if typeobj.is_bitcase:
        switch_obj = typeobj.parents[-1]
    if switch_obj is not None:
        ffi_type = switch_obj.ffi_type

    _f.section(1)
    if field.type.is_simple:
        _f('')
        _f('pub fn %s (R: *const %s)', field.ffi_accessor_fn, ffi_type)
        _f('        -> %s;', field.ffi_field_type)
    else:
        if field.type.is_switch and switch_obj is None:
            return_type = '*mut c_void'
        else:
            return_type = '*mut %s' % field.ffi_field_type

        _f('')
        _f('pub fn %s (R: *const %s)', field.ffi_accessor_fn, ffi_type)
        _f('        -> %s;', return_type)


def _ffi_accessors(typeobj, nametup):
    for field in typeobj.fields:
        if not field.type.is_pad:
            if field.type.is_list and not field.type.fixed_size():
                _ffi_accessors_list(typeobj, field)
            elif (field.prev_varsized_field is not None
                    or not field.type.fixed_size()):
                _ffi_accessors_field(typeobj, field)


def _ffi_iterator(typeobj, nametup):

    _f.section(0)
    _f('')
    _f('#[repr(C)]')
    _f('pub struct %s {', typeobj.ffi_iterator_type)
    _f('    pub data:  *mut %s,', typeobj.ffi_type)
    _f('    pub rem:   c_int,')
    _f('    pub index: c_int,')
    _f('}')

    _f.section(1)
    _f('')
    _f('pub fn %s (i: *mut %s);', typeobj.ffi_next_fn,
            typeobj.ffi_iterator_type)

    _f('')
    _f('pub fn %s (i: *mut %s)', typeobj.ffi_end_fn,
            typeobj.ffi_iterator_type)
    _f('        -> xcb_generic_iterator_t;')




def _ffi_cookie(request):
    _f.section(0)
    _f('')
    _f('#[derive(Copy, Clone)]')
    _f('#[repr(C)]')
    _f('pub struct %s {', request.ffi_cookie_type)
    _f('    sequence: c_uint')
    _f('}')


def _ffi_request_helper(request, nametup, cookie_type, void, regular,
        aux=False):
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
    func_cookie = 'xcb_void_cookie_t' if void else request.ffi_cookie_type

    # What our function name is
    func_name = request.ffi_request_fn if not aux else self.ffi_aux_fn
    if checked:
        func_name = request.ffi_checked_fn if not aux else request.ffi_aux_checked_fn
    if unchecked:
        func_name = request.ffi_unchecked_fn if not aux else request.ffi_aux_unchecked_fn

    param_fields = []
    wire_fields = []
    maxnamelen = len('c') # xcb_connection_t
    serial_fields = []
    # special case: list with variable size elements
    list_with_var_size_elems = False

    for field in request.fields:
        if field.visible:
            # The field should appear as a call parameter
            param_fields.append(field)
        if field.wire and not field.auto:
            # We need to set the field up in the structure
            wire_fields.append(field)
        if field.type.ffi_need_serialize or field.type.ffi_need_sizeof:
            serial_fields.append(field)

    for field in param_fields:
        maxnamelen = max(maxnamelen, len(field.ffi_field_name))
        if field.type.is_list and not field.type.member.fixed_size():
            list_with_var_size_elems = True


    _f.section(1)
    _f('')
    fn_start = 'pub fn %s (' % func_name
    eol = ',' if len(param_fields) else ')'
    _f('%sc: *mut xcb_connection_t%s', fn_start, eol)

    func_spacing = ' ' * len(fn_start)

    for (i, field) in enumerate(param_fields):

        param_type = field.ffi_field_type
        if field.ffi_need_pointer:
            pointer = '*const ' if field.ffi_need_const else '*mut '
            param_type = pointer + param_type

        if field.type.ffi_need_serialize and not aux:
            param_type = '*const c_void'

        spacing = ' ' * (maxnamelen - len(field.ffi_field_name))
        eol = ')' if i == (len(param_fields)-1) else ','

        _f('%s%s: %s%s%s', func_spacing, field.ffi_field_name,
                spacing, param_type, eol)

    _f('        -> %s;', cookie_type)



def _ffi_reply(request, name):
    '''
    Declares the function that returns the reply structure.
    '''
    _f.section(1)
    _f('')
    _f('/// the returned value must be freed by the caller using libc::free().')
    fn_start = 'pub fn %s (' % request.ffi_reply_fn
    spacing = ' ' * len(fn_start)
    _f('%sc:      *mut xcb_connection_t,', fn_start)
    _f('%scookie: %s,', spacing, request.ffi_cookie_type)
    _f('%serror:  *mut *mut xcb_generic_error_t)', spacing)
    _f('        -> *mut %s;', request.ffi_reply_type)



def _ffi_reply_has_fds(self):
    for field in self.fields:
        if field.isfd:
            return True
    return False


def _ffi_reply_fds(request, name):
    '''
    Declares the function that returns fds related to the reply.
    '''
    _f.section(1)
    _f('')
    _f('/// the returned value must be freed by the caller using libc::free().')
    fn_start = 'pub fn %s (' % request.ffi_reply_fds_fn
    spacing = ' ' * len(fn_start)
    _f('%sc:     *mut xcb_connection_t,', fn_start)
    _f('%sreply: %s)', spacing, request.ffi_cookie_type)
    _f('        -> *mut c_int;')



# Rust codegen function

def _rs_type_setup(typeobj, nametup):
    #assert typeobj.hasattr('ffi_type')

    typeobj.rs_type = _rs_type_name(nametup)
    typeobj.rs_has_lifetime = False

    if typeobj.is_container:
        typeobj.rs_wrap_type = 'StructPtr'
        typeobj.rs_wrap_field = 'ptr'
        typeobj.rs_has_lifetime = True

        for field in typeobj.fields:
            _rs_type_setup(field.type, field.field_type)


def _rs_struct(typeobj):

    lifetime1 = ''
    lifetime2 = ''
    if typeobj.rs_has_lifetime:
        lifetime1 = "<'a>"
        lifetime2 = "'a, "

    _r.section(0)
    _r('')
    _r('pub struct %s%s {', typeobj.rs_type, lifetime1)
    _r('    pub base: base::%s<%s%s>', typeobj.rs_wrap_type,
                lifetime2, typeobj.ffi_type)
    _r('}')


# Common codegen functions

class EnumCodegen(object):

    namecount = {}

    def build_collision_table(module):
        for v in module.types.values():
            key = _ffi_type_name(v[0])
            EnumCodegen.namecount[key] = (
                (EnumCodegen.namecount.get(key) or 0) + 1
            )


    def __init__(self, nametup):
        self._nametup = nametup

        self.done_vals = {}
        self.discriminants = []
        self.conflicts = []
        key = _ffi_type_name(nametup)
        if EnumCodegen.namecount[key] > 1:
            nametup = nametup + ('enum',)
        self.ffi_name = _ffi_type_name(nametup)
        self.rs_name = _rs_type_name(nametup)


    def add_discriminant(self, name, val):
        class Discriminant: pass
        d = Discriminant()
        d.rs_name = name
        if d.rs_name[0].isdigit():
            d.rs_name = '_' + d.rs_name
        d.ffi_name = _upper_name(_ext_nametup(self._nametup+(name,)))
        d.valstr = '0x%02x' % val
        d.val = val
        if val in self.done_vals:
            self.conflicts.append(d)
        else:
            self.done_vals[val] = d
            self.discriminants.append(d)

    def gen_code(self, sf, name_field, reprC):
        maxnamelen = 0
        maxvallen = 0
        for d in self.discriminants:
            maxvallen = max(maxvallen, len(d.valstr))
            maxnamelen = max(maxnamelen, len(getattr(d, name_field)))

        sf.section(0)
        sf('')
        if reprC and len(self.discriminants) > 1:
            sf('#[repr(C)]')
        sf('pub enum %s {', getattr(self, name_field))
        sf.indent()
        for d in self.discriminants:
            dname = getattr(d, name_field)
            namespace = ' ' * (maxnamelen-len(dname))
            valspace = ' ' * (maxvallen-len(d.valstr))
            sf('%s %s= %s%s,', dname, namespace, valspace, d.valstr)
        sf.unindent()
        sf('}')



# codegen drivers

def rs_simple(simple, nametup):
    '''
    simple is SimpleType object
    nametup is a name tuple
    '''
    print('simple:  ', nametup)

    _ffi_type_setup(simple, nametup)
    _f.section(0)
    assert len(simple.name) == 1
    _f('')
    _f('pub type %s = %s;', simple.ffi_type, simple.name[0])
    _ffi_iterator(simple, nametup)

    _rs_type_setup(simple, nametup)
    _r.section(0)
    _r('')
    _r('pub type %s = %s;', simple.rs_type, simple.ffi_type)



def rs_enum(typeobj, nametup):
    '''
    typeobj is xcbgen.xtypes.Enum object
    nametup is a name tuple
    '''
    print('enum:    ', nametup)

    ecg = EnumCodegen(nametup)

    val = -1
    for (enam, eval) in typeobj.values:
        val = int(eval) if eval != '' else val+1
        ecg.add_discriminant(enam, val)

    ecg.gen_code(_f, "ffi_name", True)
    ecg.gen_code(_r, "rs_name", False)

    # writing conflicts after only for FFI at the moment
    if len(ecg.conflicts):
        _f('')
    for c in ecg.conflicts:
        d = ecg.done_vals[c.val]
        _f('pub const %s: %s = %s::%s;',
               c.ffi_name, ecg.ffi_name, ecg.ffi_name, d.ffi_name)



def rs_struct(struct, nametup):
    '''
    struct is Struct object
    nametup is a name tuple
    '''
    print('struct:  ', nametup)
    _ffi_type_setup(struct, nametup)
    _ffi_struct(struct)
    _ffi_accessors(struct, nametup)
    _ffi_iterator(struct, nametup)

    _rs_type_setup(struct, nametup)
    _rs_struct(struct)

def rs_union(union, nametup):
    '''
    union is Union object
    nametup is a name tuple
    '''
    print('union:   ', nametup)
    _ffi_type_setup(union, nametup)

    biggest = 1
    most_aligned = 1
    ptr_size = 8 if sys.maxsize > 2**32 else 4
    for field in union.fields:
        fs = ptr_size
        fa = ptr_size
        if field.type.size:
            fs = field.type.size
            fa = field.type.size
        if field.type.nmemb:
            fs = fa * field.type.nmemb
        biggest = max(biggest, fs)
        most_aligned = max(most_aligned, fa)

    assert biggest >= most_aligned

    num_aligned = int(biggest / most_aligned)
    if biggest % most_aligned:
        num_aligned += 1

    num_bytes = num_aligned * most_aligned

    _f.section(0)
    _f('')
    _f('#[repr(C)]')
    _f('pub struct %s {', union.ffi_type)
    _f('    pub data: [u8; %s]', num_bytes)
    _f('}')



def rs_request(request, nametup):
    '''
    request is Request object
    nametup is a name tuple
    '''
    print('request: ', nametup)
    _ffi_type_setup(request, nametup, ('request',))

    if request.reply:
        _ffi_cookie(request)

    _ffi_opcode(nametup, request.opcode)

    _ffi_struct(request)

    if request.reply:
        _ffi_type_setup(request.reply, nametup, ('reply',))
        _ffi_struct(request.reply)
        _ffi_request_helper(request, nametup, request.ffi_cookie_type,
                False, True, False)
        _ffi_request_helper(request, nametup, request.ffi_cookie_type,
                False, False, False)
        if request.ffi_need_aux:
            _ffi_request_helper(request, nametup, request.ffi_cookie_type,
                    False, True, True)
            _ffi_request_helper(request, nametup, request.ffi_cookie_type,
                    False, False, True)
        _ffi_accessors(request.reply, nametup + ('reply',))
        _ffi_reply(request, nametup)
        if _ffi_reply_has_fds(request.reply):
            _ffi_reply_fds(request, nametup)
    else:
        _ffi_request_helper(request, nametup, 'xcb_void_cookie_t',
                True, False)
        _ffi_request_helper(request, nametup, 'xcb_void_cookie_t',
                True, True)
        if request.ffi_need_aux:
            _ffi_request_helper(request, nametup, 'xcb_void_cookie_t',
                    True, False, True)
            _ffi_request_helper(request, nametup, 'xcb_void_cookie_t',
                    True, True, True)



def rs_event(event, nametup):
    '''
    event is Event object
    nametup is a name tuple
    '''
    print('event:   ', nametup)

    # The generic event structure xcb_ge_event_t has the full_sequence field
    # at the 32byte boundary. That's why we've to inject this field into GE
    # events while generating the structure for them. Otherwise we would read
    # garbage (the internal full_sequence) when accessing normal event fields
    # there.
    must_pack = False
    if (hasattr(event, 'is_ge_event')
            and event.is_ge_event
            and event.name == nametup):
        event_size = 0
        for field in event.fields:
            if field.type.size != None and field.type.nmemb != None:
                event_size += field.type.size * field.type.nmemb
            if event_size == 32:
                full_sequence = Field(tcard32,
                        tcard32.name, 'full_sequence',
                        False, True, True)
                idx = event.fields.index(field)
                event.fields.insert(idx + 1, full_sequence)

                # If the event contains any 64-bit extended fields, they need
                # to remain aligned on a 64-bit boundary.  Adding full_sequence
                # would normally break that; force the struct to be packed.
                must_pack = any(f.type.size == 8 and f.type.is_simple
                        for f in event.fields[(idx+1):])
                break

    if must_pack:
        print('event ', nametup, ' is packed')

    _ffi_type_setup(event, nametup, ('event',))
    _ffi_opcode(nametup, event.opcodes[nametup])

    if event.name == nametup:
        _ffi_struct(event, must_pack)
    else:
        _f.section(0)
        _f('')
        _f('type %s = %s;', _ffi_type_name(nametup+('event',)),
                            _ffi_type_name(event.name+('event',)))



def rs_error(error, nametup):
    '''
    error is Error object
    nametup is a name tuple
    '''
    print('error:   ', nametup)
    _ffi_type_setup(error, nametup, ('error',))
    _ffi_opcode(nametup, error.opcodes[nametup])

    if error.name == nametup:
        _ffi_struct(error)
    else:
        _f.section(0)
        _f('')
        _f('type %s = %s;', _ffi_type_name(nametup+('error',)),
                            _ffi_type_name(error.name+('error',)))
    pass


def usage(program):
    print(stderr, 'Usage: {} -o SRCDIR file.xml', program)


if __name__ == '__main__':

    from optparse import OptionParser

    parser = OptionParser(usage="Usage: %prog -o SRCDIR file.xml")
    parser.add_option('-o', '--output', dest='srcdir', metavar='SRCDIR',
                help='specifies rust src dir where to generate files')

    (options, args) = parser.parse_args(sys.argv)

    if options.srcdir == None:
        parser.error('-o SRCDIR is mandatory')

    if not os.path.isdir(options.srcdir):
        parser.error('-o SRCDIR must be a directory')

    if len(args) < 2:
        parser.error('input XML file must be supplied')

    output = {  'open'      : rs_open,
                'close'     : rs_close,
                'simple'    : rs_simple,
                'enum'      : rs_enum,
                'struct'    : rs_struct,
                'union'     : rs_union,
                'request'   : rs_request,
                'event'     : rs_event,
                'error'     : rs_error }
    try:
        from xcbgen.state import Module
        from xcbgen.xtypes import *
    except ImportError:
        print(stderr, 'failed to load xcbgen')
        raise

    # Parse the xml header
    module = Module(args[1], output)
    module.rs_srcdir = options.srcdir

    # Build type-registry and resolve type dependencies
    module.register()
    module.resolve()

    # Output the code
    module.generate()

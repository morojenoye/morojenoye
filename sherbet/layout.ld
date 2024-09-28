MEMORY
{
	R0 : ORIGIN = 0x80080000, LENGTH = 8M
}

PROVIDE( _HART_STACK_SIZE = 2K );
PROVIDE( _HART_COUNT = 2 );

_stack_0 = ALIGN( ORIGIN ( R0 ) + _HART_STACK_SIZE * _HART_COUNT, 8 );
_stack_1 = ORIGIN ( R0 );

ENTRY( start )

SECTIONS
{
	.text _stack_0 : {
		* ( .text.start )
		* ( .rodata .rodata.* )
		* ( .text .text.* )
		. = ALIGN( 256 );
		* ( .text.interrupt )
	} > R0

	.data : {
		PROVIDE( __global_pointer$ = . + 2K );
		* ( .sdata .sdata.* )
		* ( .data .data.* )
	} > R0

	.wipe ( NOLOAD ) : {
		_wipe_0 = .;
		* ( .sbss .sbss.* )
		* ( .bss .bss.* )
		_wipe_1 = .;
	} > R0
}

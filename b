The following options are target specific:
  -m128bit-long-double        		[disabled]
  -m16                        		[disabled]
  -m32                        		[disabled]
  -m3dnow                     		[disabled]
  -m3dnowa                    		[disabled]
  -m64                        		[enabled]
  -m80387                     		[enabled]
  -m8bit-idiv                 		[disabled]
  -m96bit-long-double         		[enabled]
  -mabi=                      		sysv
  -mabm                       		[disabled]
  -maccumulate-outgoing-args  		[disabled]
  -maddress-mode=             		short
  -madx                       		[disabled]
  -maes                       		[disabled]
  -malign-data=               		compat
  -malign-double              		[disabled]
  -malign-functions=          		0
  -malign-jumps=              		0
  -malign-loops=              		0
  -malign-stringops           		[enabled]
  -mandroid                   		[disabled]
  -march=                     		x86-64
  -masm=                      		att
  -mavx                       		[disabled]
  -mavx2                      		[disabled]
  -mavx256-split-unaligned-load 	[disabled]
  -mavx256-split-unaligned-store 	[disabled]
  -mavx512bw                  		[disabled]
  -mavx512cd                  		[disabled]
  -mavx512dq                  		[disabled]
  -mavx512er                  		[disabled]
  -mavx512f                   		[disabled]
  -mavx512ifma                		[disabled]
  -mavx512pf                  		[disabled]
  -mavx512vbmi                		[disabled]
  -mavx512vl                  		[disabled]
  -mbionic                    		[disabled]
  -mbmi                       		[disabled]
  -mbmi2                      		[disabled]
  -mbranch-cost=              		0
  -mcld                       		[disabled]
  -mclflushopt                		[disabled]
  -mclwb                      		[disabled]
  -mcmodel=                   		32
  -mcpu=                      		
  -mcrc32                     		[disabled]
  -mcx16                      		[disabled]
  -mdispatch-scheduler        		[disabled]
  -mdump-tune-features        		[disabled]
  -mf16c                      		[disabled]
  -mfancy-math-387            		[enabled]
  -mfentry                    		[enabled]
  -mfma                       		[disabled]
  -mfma4                      		[disabled]
  -mforce-drap                		[disabled]
  -mfp-ret-in-387             		[enabled]
  -mfpmath=                   		387
  -mfsgsbase                  		[disabled]
  -mfused-madd                		
  -mfxsr                      		[disabled]
  -mglibc                     		[enabled]
  -mhard-float                		[enabled]
  -mhle                       		[disabled]
  -mieee-fp                   		[enabled]
  -mincoming-stack-boundary=  		0
  -minline-all-stringops      		[disabled]
  -minline-stringops-dynamically 	[disabled]
  -mintel-syntax              		
  -mlarge-data-threshold=     		0x10000
  -mlong-double-128           		[disabled]
  -mlong-double-64            		[disabled]
  -mlong-double-80            		[enabled]
  -mlwp                       		[disabled]
  -mlzcnt                     		[disabled]
  -mmemcpy-strategy=          		
  -mmemset-strategy=          		
  -mmmx                       		[disabled]
  -mmovbe                     		[disabled]
  -mmpx                       		[disabled]
  -mms-bitfields              		[disabled]
  -mmwaitx                    		[disabled]
  -mno-align-stringops        		[disabled]
  -mno-default                		[disabled]
  -mno-fancy-math-387         		[disabled]
  -mno-push-args              		[disabled]
  -mno-red-zone               		[disabled]
  -mno-sse4                   		[enabled]
  -mnop-mcount                		[disabled]
  -momit-leaf-frame-pointer   		[disabled]
  -mpc32                      		[disabled]
  -mpc64                      		[disabled]
  -mpc80                      		[disabled]
  -mpclmul                    		[disabled]
  -mpcommit                   		[disabled]
  -mpopcnt                    		[disabled]
  -mprefer-avx128             		[disabled]
  -mpreferred-stack-boundary= 		0
  -mprefetchwt1               		[disabled]
  -mprfchw                    		[disabled]
  -mpush-args                 		[enabled]
  -mrdrnd                     		[disabled]
  -mrdseed                    		[disabled]
  -mrecip                     		[disabled]
  -mrecip=                    		
  -mrecord-mcount             		[disabled]
  -mred-zone                  		[enabled]
  -mregparm=                  		0
  -mrtd                       		[disabled]
  -mrtm                       		[disabled]
  -msahf                      		[disabled]
  -msha                       		[disabled]
  -mskip-rax-setup            		[disabled]
  -msoft-float                		[disabled]
  -msse                       		[disabled]
  -msse2                      		[disabled]
  -msse2avx                   		[disabled]
  -msse3                      		[disabled]
  -msse4                      		[disabled]
  -msse4.1                    		[disabled]
  -msse4.2                    		[disabled]
  -msse4a                     		[disabled]
  -msse5                      		
  -msseregparm                		[disabled]
  -mssse3                     		[disabled]
  -mstack-arg-probe           		[disabled]
  -mstack-protector-guard=    		tls
  -mstackrealign              		[enabled]
  -mstringop-strategy=        		[default]
  -mtbm                       		[disabled]
  -mtls-dialect=              		gnu
  -mtls-direct-seg-refs       		[enabled]
  -mtune-ctrl=                		
  -mtune=                     		generic
  -muclibc                    		[disabled]
  -mveclibabi=                		[default]
  -mvect8-ret-in-mem          		[disabled]
  -mvzeroupper                		[disabled]
  -mx32                       		[disabled]
  -mxop                       		[disabled]
  -mxsave                     		[disabled]
  -mxsavec                    		[disabled]
  -mxsaveopt                  		[disabled]
  -mxsaves                    		[disabled]

  Known assembler dialects (for use with the -masm-dialect= option):
    att intel

  Known ABIs (for use with the -mabi= option):
    ms sysv

  Known code models (for use with the -mcmodel= option):
    32 kernel large medium small

  Valid arguments to -mfpmath=:
    387 387+sse 387,sse both sse sse+387 sse,387

  Known data alignment choices (for use with the -malign-data= option):
    abi cacheline compat

  Known vectorization library ABIs (for use with the -mveclibabi= option):
    acml svml

  Known address mode (for use with the -maddress-mode= option):
    long short

  Known stack protector guard (for use with the -mstack-protector-guard= option):
    global tls

  Valid arguments to -mstringop-strategy=:
    byte_loop libcall loop rep_4byte rep_8byte rep_byte unrolled_loop
    vector_loop

  Known TLS dialects (for use with the -mtls-dialect= option):
    gnu gnu2


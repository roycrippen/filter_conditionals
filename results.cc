Target identifiers: 
  [ 
    NRTSIM
    RTCLSIM
  ]

Affirmative targets: 
  [ 
    #if defined(_DEBUG) && (NRTSIM || RTCLSIM)
    #if (NRTSIM || RTCLSIM || _DEBUG)
    #if defined(_DEBUG) && ( NRTSIM || RTCLSIM )
    #if (NRTSIM)
    #if ( NRTSIM )
    #ifdef NRTSIM /* If on desktop, redefine types to match new systypes. */
    #ifdef NRTSIM
    #if (NRTSIM || RTCLSIM)
    #if (defined NRTSIM)
    #if ( NRTSIM || RTCLSIM )
    #if (defined CPU_B) || (defined NRTSIM)
  ]

Non-affirmative targets: 
  [ 
    #if (defined CPU_B) && (!defined NRTSIM)          
    #if ( !NRTSIM && !RTCLSIM )                       
    #if (!NRTSIM && !RTCLSIM)                         
    #if (!NRTSIM )                                    
    #if (!NRTSIM || _TBBVSIM_ ) // Not needed for desktop simulation
    #if !(NRTSIM || RTCLSIM)                          
    #if ( !NRTSIM )                                   
    #if !defined(_DEBUG) || !( NRTSIM || RTCLSIM )    
    #if (!NRTSIM)                                     
    #if (!defined CPU_B) && (!defined NRTSIM)         
    #if (defined WINDVIEW_DEBUG) && (!defined NRTSIM) 
    #if (!defined NRTSIM)                             
    #ifndef RTCLSIM  
    #if (!NRTSIM) // Not needed for desktop simulation
    #if !( NRTSIM || RTCLSIM )                        
    #if ((NRTSIM || RTCLSIM) && (!__PPC604__))        
    #if !(NRTSIM)                                     
    #ifndef NRTSIM                                    
    #if (!defined CPU_B) || (defined NRTSIM)          
  ]

Affirmative targets:
#if ( NRTSIM )
#if ( NRTSIM || RTCLSIM )
#if (NRTSIM || RTCLSIM || _DEBUG)
#if (NRTSIM || RTCLSIM)
#if (NRTSIM)
#if (defined CPU_B) || (defined NRTSIM)
#if (defined NRTSIM)
#if defined(_DEBUG) && ( NRTSIM || RTCLSIM )
#if defined(_DEBUG) && (NRTSIM || RTCLSIM)
#ifdef NRTSIM
#ifdef NRTSIM /* If on desktop, redefine types to match new systypes. */

Non-affirmative targets:
#if !( NRTSIM || RTCLSIM )
#if !(NRTSIM || RTCLSIM)
#if !(NRTSIM)
#if !defined(_DEBUG) || !( NRTSIM || RTCLSIM )
#if ( !NRTSIM && !RTCLSIM )
#if ( !NRTSIM )
#if (!NRTSIM && !RTCLSIM)
#if (!NRTSIM )
#if (!NRTSIM || _TBBVSIM_ ) // Not needed for desktop simulation
#if (!NRTSIM)
#if (!NRTSIM) // Not needed for desktop simulation
#if (!defined CPU_B) && (!defined NRTSIM)
#if (!defined CPU_B) || (defined NRTSIM)
#if (!defined NRTSIM)
#if ((NRTSIM || RTCLSIM) && (!__PPC604__))
#if (defined CPU_B) && (!defined NRTSIM)
#if (defined WINDVIEW_DEBUG) && (!defined NRTSIM)
#ifndef NRTSIM
#ifndef RTCLSIM

/home/crippenre/dev/booster/OBV2_4_SC/BSPRad750/BSP_API.h
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cAutoPilot/test/AcsAutopilot_test/src/AcsAutopilot_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cAutoPilot/test/AlphaLoop_test/src/AlphaLoop_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cAutoPilot/test/InnerLoop_test/src/InnerLoop_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cAutoPilot/test/InnerLoop_test/src/InnerLoop_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cAutoPilot/test/OpenLoop_test/src/OpenLoopGuidance_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cAutoPilot/test/OpenLoop_test/src/OpenLoopGuidance_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cAutoPilot/test/OpenLoop_test/src/OpenLoopGuidance_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cAutoPilot/test/OuterLoop_test/src/OuterLoop_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cFilter/test/AccelFilter_test/src/AccelFilter_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cFilter/test/RateFilter_test/src/RateFilter_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/src/lambertScrubGuide.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/src/lambertScrubGuide.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/src/lambertScrubGuide.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/src/lambertScrubGuide.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/src/lambertScrubGuide.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/src/lambertScrubGuide.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/src/lambertScrubGuide.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/src/lambertScrubGuide.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/src/lambertScrubGuide.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/src/lambertScrubGuide.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/src/lambertScrubGuide.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/src/lambertScrubGuide.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/src/lambertScrubGuide.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/src/lambertScrubGuide.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/src/lambertScrubGuide.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/src/lambertScrubGuide.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/src/lambertScrubGuide.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/test/LambertScrubGuide_test/src/LambertScrubGuide_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/test/LambertScrubGuide_test/src/LambertScrubGuide_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/test/LambertScrubGuide_test/src/LambertScrubGuide_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/test/LambertScrubGuide_test/src/LambertScrubGuide_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/test/LambertScrubGuide_test/src/LambertScrubGuide_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/test/LambertScrubGuide_test/src/LambertScrubGuide_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/test/LambertScrubGuide_test/src/LambertScrubGuide_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/test/LambertScrubGuide_test/src/LambertScrubGuide_test.hh
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cGuidance/test/VelocitySteering_test/src/VelocitySteering_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/cNavigator/src/NexGenSigiRT.cc
/home/crippenre/dev/booster/OBV2_4_SC/Flight/include/flightConditions.hh
/home/crippenre/dev/booster/OBV2_4_SC/Flight/include/flightConditions.hh
/home/crippenre/dev/booster/OBV2_4_SC/Flight/include/flightConditions.hh
/home/crippenre/dev/booster/OBV2_4_SC/Flight/include/hwdevice.hh
/home/crippenre/dev/booster/OBV2_4_SC/Flight/include/lambertScrubGuide.hh
/home/crippenre/dev/booster/OBV2_4_SC/Flight/include/que.hh
/home/crippenre/dev/booster/OBV2_4_SC/Flight/include/timeBase.hh
/home/crippenre/dev/booster/OBV2_4_SC/Flight/include/watchdog.hh
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cActuator/test/target/TvcActuator_test/src/TvcActuator_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cActuator/test/target/TvcActuator_test/src/TvcActuator_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cBAN/src/banCovMgr.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cBAN/src/banCovMgr.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cBAN/src/banCovMgr.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cBAN/src/banCovMgr.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cBAN/src/banCovMgr.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cBAN/src/banCovMgr.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cBAN/src/bandata.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cBit/src/StartUpBit.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cBit/src/StartUpBit.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cCle/src/CleTxMessageFactory.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cDiscrete/src/Discrete.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cExecutive/src/XRAM.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cExecutive/src/XRAM.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cExecutive/test/target/xramTest/ASideTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cExecutive/test/target/xramTest/ASideTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cExecutive/test/target/xramTest/ASideTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cExecutive/test/target/xramTest/ASideTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cExecutive/test/target/xramTest/ASideTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cExecutive/test/target/xramTest/ASideTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cExecutive/test/target/xramTest/ASideTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cExecutive/test/target/xramTest/BSideTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cExecutive/test/target/xramTest/BSideTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cExecutive/test/target/xramTest/BSideTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cExecutive/test/target/xramTest/BSideTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cTest/src/unitTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cTest/src/unitTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cTest/src/unitTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cTest/src/xTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cTest/src/xTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cTest/src/xTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cTest/src/xTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cTest/src/xTest.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cTlm/src/idents.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cUserCommand/src/DataPages.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/flight/cUserCommand/src/DataPages.cc
/home/crippenre/dev/booster/OBV2_4_SC/OBV/include/banCovMgr.hh
/home/crippenre/dev/booster/OBV2_4_SC/OBV/include/banCovMgr.hh
/home/crippenre/dev/booster/OBV2_4_SC/OBV/include/CleTxMessageFactory.hh
/home/crippenre/dev/booster/OBV2_4_SC/OBV/include/ObvState.hh
/home/crippenre/dev/booster/OBV2_4_SC/OBV/include/ObvState.hh
/home/crippenre/dev/booster/OBV2_4_SC/OBV/include/SimVxWorks.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cDataBase/src/database.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cDataBase/src/database.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cDataBase/src/database.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cDataBase/src/database.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cDataBase/test/src/database_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cDataBase/test/src/database_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cDataBase/test/src/database_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cFilter/src/filter.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cFilter/src/filter.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cFilter/src/filter.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cFilter/src/filter.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cFilter/src/filter.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cFilter/src/filter.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cFilter/src/filter.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cMath/src/genarray.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cMath/src/genarray.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cMath/src/matrix.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cMath/src/matsolve.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cMath/src/matsolve.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cMath/src/quaternion.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cMath/src/quaternion.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cMath/src/rqlsqfit.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cMath/src/transform.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cMath/src/transform1.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cMath/src/transform2.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cMath/src/transform23.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cMath/src/transform3.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cMath/src/transform32.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cMath/test/Matrix3/src/Matrix3_test.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cNav/src/navtools.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cOutput/src/simoutput.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cOutput/src/simouttypes.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cTable/src/table.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cTable/src/table.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cTable/src/table.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cTable/src/table.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cTable/src/table.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cTable/src/table.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cTable/src/table.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cTable/src/table.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cTable/src/table.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cTable/src/table.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cTable/src/TableList.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/cTable/src/TableList.cc
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/gvector.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/gvector.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/list.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/list.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/list.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/list.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/list.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/Mask.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/MaskedMatrix.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/MaskedVector.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/matrix.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/matrix3.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/matrix3.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/MatrixOperations.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/OscMemory.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/OscMemory.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/OscMemory.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/OscMemory.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/output.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/quaternion.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/quaternion.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/quaternion.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/quaternion.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/quaternion.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/vector.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/vector3.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/vector3.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/vector3.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/vector3.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/vector3.hh
/home/crippenre/dev/booster/OBV2_4_SC/Utility/include/VectorOperations.hh
/home/crippenre/dev/booster/OBV2_4_SC/test.cc

Summary Results
  Total files:      934
  Affected files:     0
  Blocks removed:     0
  Lines removed:      0

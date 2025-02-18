void Assets_Scripts_Simulation_Towers_Behaviors_Attack_Behaviors_WingmonkeyPattern__UpdateMovement(
        Assets_Scripts_Simulation_Towers_Behaviors_Attack_Behaviors_WingmonkeyPattern_o *this,
        float delta,
        const MethodInfo *method)
{
  __int64 v3; // rdx
  __int64 v6; // rdx
  __int64 v7; // rdx
  Assets_Scripts_Simulation_SMath_Vector3_c *v8; // rax
  bool v9; // zf
  struct Assets_Scripts_Simulation_SMath_Vector3_StaticFields *static_fields; // rax
  unsigned __int64 v11; // xmm1_8
  float z; // ecx
  struct Assets_Scripts_Simulation_Bloons_Bloon_o *v13; // rax
  struct Assets_Scripts_Models_Towers_Behaviors_Attack_Behaviors_WingmonkeyPatternModel_o *wingmonkeyPatternModel; // r8
  Assets_Scripts_Simulation_Track_Path_o *path; // rcx
  float overshootDist; // xmm6_4
  float maxPossibleDist; // xmm7_4
  bool isInBoundsOfPath; // al
  struct Assets_Scripts_Simulation_Bloons_Bloon_o *currentBloonTarget; // rcx
  Assets_Scripts_Simulation_Track_Path_o *_path; // rdx
  Assets_Scripts_Simulation_SMath_Vector3_o *v21; // rax
  Assets_Scripts_Simulation_SMath_Vector3Boxed_o *bloon_pos_maybe; // rax
  Assets_Scripts_Simulation_SMath_Vector2_o bloon_pos_vec2; // rax
  Assets_Scripts_Simulation_Bloons_Bloon_o *currBloonTarget; // rcx
  Unity_Mathematics_float2_o bloon_pos_float2; // xmm8_8
  Assets_Scripts_Simulation_SMath_Vector2_o v26; // rdx
  Assets_Scripts_Simulation_SMath_Vector2_o v27; // r8
  Assets_Scripts_Simulation_SMath_Vector2_o Heading; // xmm7_8
  Unity_Mathematics_float2_o scaled_bloon_heading; // rax
  __m128 v30; // xmm1
  float v31; // eax
  struct Assets_Scripts_Simulation_Towers_Behaviors_AirUnit_o *airUnit; // rax
  struct Assets_Scripts_Simulation_Behaviors_TransformBehavior_o *transform; // rcx
  UnityEngine_ResourceManagement_Util_DelayedActionManager_DelegateInfo_o *position; // rcx
  __int64 v35; // rdx
  __int64 v36; // r8
  Unity_Mathematics_float2_o xy; // xmm7_8
  struct Assets_Scripts_Simulation_Towers_Behaviors_AirUnit_o *v38; // rax
  struct Assets_Scripts_Simulation_Behaviors_TransformBehavior_o *v39; // rax
  Assets_Scripts_Simulation_SMath_Vector3Boxed_o *v40; // rcx
  Assets_Scripts_Simulation_SMath_Vector2_o v41; // rdx
  Assets_Scripts_Simulation_SMath_Vector2_o v42; // r8
  Assets_Scripts_Simulation_SMath_Vector2_o v43; // xmm6_8
  __int64 v44; // rdx
  __int64 v45; // r8
  Assets_Scripts_Simulation_SMath_Vector2_o v46; // rax
  struct Assets_Scripts_Simulation_Bloons_Bloon_o *v47; // rcx
  Unity_Mathematics_float2_o v48; // xmm6_8
  __int64 v49; // rax
  __int64 v50; // xmm7_8
  float v51; // edi
  Assets_Scripts_Simulation_SMath_Vector3_o *v52; // rax
  __int64 v53; // rdx
  __int64 v54; // r8
  __int64 v55; // xmm6_8
  float v56; // esi
  Unity_Mathematics_float3_o *v57; // rax
  __int64 v58; // xmm1_8
  float v59; // ecx
  float v60; // eax
  struct Assets_Scripts_Simulation_Towers_Behaviors_AirUnit_o *v61; // rcx
  UnityEngine_ResourceManagement_Util_DelayedActionManager_DelegateInfo_o *v62; // rax
  float InvocationTime; // xmm0_4
  struct Assets_Scripts_Simulation_Towers_Behaviors_AirUnit_o *v64; // rax
  struct Assets_Scripts_Simulation_Behaviors_TransformBehavior_o *v65; // rax
  Assets_Scripts_Simulation_SMath_Vector3Boxed_o *v66; // rcx
  Assets_Scripts_Simulation_Bloons_Bloon_o *BloonTarget; // rax
  Assets_Scripts_Simulation_SMath_Vector3_o *RandomFlyoverPosition; // rax
  float v69; // ecx
  __int64 v70; // xmm0_8
  struct Assets_Scripts_Models_Towers_Behaviors_Attack_Behaviors_WingmonkeyPatternModel_o *v71; // rax
  Unity_Mathematics_float4_o v72; // [rsp+20h] [rbp-39h] BYREF
  Unity_Mathematics_float3_o v73; // [rsp+30h] [rbp-29h] BYREF
  Assets_Scripts_Simulation_SMath_Vector3_o point_on_path; // [rsp+40h] [rbp-19h] BYREF
  Unity_Mathematics_float3_o v75; // [rsp+50h] [rbp-9h] BYREF
  Unity_Mathematics_float2_o extrapolated_bloon_pos; // [rsp+C0h] [rbp+67h] BYREF
  Assets_Scripts_Simulation_SMath_Vector2_o v77; // [rsp+D8h] [rbp+7Fh] BYREF

  if ( !byte_183E25BD5 )
  {
    sub_1802D3590(&Method_Assets_Scripts_Simulation_Physics_Collidable_IBloonBehavior__get_Radius__, v3);
    sub_1802D3590(&Assets_Scripts_Simulation_SMath_Vector2_TypeInfo, v6);
    sub_1802D3590(&Assets_Scripts_Simulation_SMath_Vector3_TypeInfo, v7);
    byte_183E25BD5 = 1;
  }
  v77 = 0LL;
  extrapolated_bloon_pos = 0LL;
  v8 = Assets_Scripts_Simulation_SMath_Vector3_TypeInfo;
  if ( !Assets_Scripts_Simulation_SMath_Vector3_TypeInfo->_2.cctor_finished )
  {
    il2cpp_runtime_class_init(Assets_Scripts_Simulation_SMath_Vector3_TypeInfo, v3, method);
    v8 = Assets_Scripts_Simulation_SMath_Vector3_TypeInfo;
  }
  v9 = this->fields.currentBloonTarget == 0LL;
  static_fields = v8->static_fields;
  v11 = *(_QWORD *)&static_fields->zero.fields.x;
  z = static_fields->zero.fields.z;
  LODWORD(v72.fields.y) = _mm_shuffle_ps((__m128)v11, (__m128)v11, 85).m128_u32[0];
  LODWORD(v72.fields.x) = v11;
  *(_QWORD *)&point_on_path.fields.x = v11;
  v72.fields.z = z;
  if ( v9 || (v13 = this->fields.currentBloonTarget, v13->fields.isDestroyed) )
  {
    if ( this->fields.flyoverEngaged )
    {
      v60 = this->fields.flyoverDestination.fields.z;
      *(_QWORD *)&v72.fields.x = *(_QWORD *)&this->fields.flyoverDestination.fields.x;
      v72.fields.z = v60;
    }
  }
  else
  {
    if ( !v13 )
      goto LABEL_52;
    wingmonkeyPatternModel = this->fields.wingmonkeyPatternModel;
    if ( !wingmonkeyPatternModel )
      goto LABEL_52;
    path = v13->fields._path;
    if ( !path )
      goto LABEL_52;
    overshootDist = (float)(v13->fields._Radius_k__BackingField * 0.5)
                  + wingmonkeyPatternModel->fields.overshootDistance;
    maxPossibleDist = overshootDist + v13->fields.distanceTraveled;
    isInBoundsOfPath = Assets_Scripts_Simulation_Track_Path__IsInBoundsOfPath(path, maxPossibleDist, 0LL);
    currentBloonTarget = this->fields.currentBloonTarget;
    if ( isInBoundsOfPath )
    {
      if ( !currentBloonTarget )
        goto LABEL_52;
      _path = currentBloonTarget->fields._path;
      if ( !_path )
        goto LABEL_52;
      v21 = Assets_Scripts_Simulation_Track_Path__DistanceToPoint(&point_on_path, _path, maxPossibleDist, 0LL);
    }
    else
    {
      if ( !currentBloonTarget )
        goto LABEL_52;
      bloon_pos_maybe = (Assets_Scripts_Simulation_SMath_Vector3Boxed_o *)((__int64 (__fastcall *)(struct Assets_Scripts_Simulation_Bloons_Bloon_o *, const MethodInfo *))currentBloonTarget->klass->vtable._106_get_Position.methodPtr)(
                                                                            currentBloonTarget,
                                                                            currentBloonTarget->klass->vtable._106_get_Position.method);
      if ( !bloon_pos_maybe )
        goto LABEL_52;
      bloon_pos_vec2 = Assets_Scripts_Simulation_SMath_Vector3Boxed__ToVector2(bloon_pos_maybe, 0LL);
      currBloonTarget = this->fields.currentBloonTarget;
      bloon_pos_float2 = (Unity_Mathematics_float2_o)bloon_pos_vec2;
      if ( !currBloonTarget )
        goto LABEL_52;
      Heading = Assets_Scripts_Simulation_Bloons_Bloon__get_Heading(currBloonTarget, 0LL);
      if ( !Assets_Scripts_Simulation_SMath_Vector2_TypeInfo->_2.cctor_finished )
        ((void (__fastcall *)(_QWORD, _QWORD, _QWORD))il2cpp_runtime_class_init)(
          Assets_Scripts_Simulation_SMath_Vector2_TypeInfo,
          v26,
          v27);
      scaled_bloon_heading = Unity_Mathematics_float2__op_Multiply_6450629648(
                               (Unity_Mathematics_float2_o)Heading,
                               overshootDist,
                               0LL);
      extrapolated_bloon_pos = Unity_Mathematics_float2__op_Addition(bloon_pos_float2, scaled_bloon_heading, 0LL);
      v21 = Assets_Scripts_Simulation_SMath_Vector2__ToVector3(
              &point_on_path,
              (Assets_Scripts_Simulation_SMath_Vector2_o)&extrapolated_bloon_pos,
              0LL);
    }
    v30 = (__m128)*(unsigned __int64 *)&v21->fields.x;
    v31 = v21->fields.z;
    LODWORD(v72.fields.y) = _mm_shuffle_ps(v30, v30, 85).m128_u32[0];
    LODWORD(v72.fields.x) = v30.m128_i32[0];
    *(_QWORD *)&point_on_path.fields.x = v30.m128_u64[0];
    point_on_path.fields.z = v31;
    airUnit = this->fields.airUnit;
    if ( !airUnit )
      goto LABEL_52;
    transform = airUnit->fields.transform;
    if ( !transform )
      goto LABEL_52;
    position = (UnityEngine_ResourceManagement_Util_DelayedActionManager_DelegateInfo_o *)transform->fields.position;
    if ( !position )
      goto LABEL_52;
    v72.fields.z = UnityEngine_ResourceManagement_Util_DelayedActionManager_DelegateInfo__get_InvocationTime(
                     position,
                     0LL);
    if ( !Assets_Scripts_Simulation_SMath_Vector3_TypeInfo->_2.cctor_finished )
      il2cpp_runtime_class_init(Assets_Scripts_Simulation_SMath_Vector3_TypeInfo, v35, v36);
    xy = Unity_Mathematics_float4__get_xy(&v72, 0LL);
    v38 = this->fields.airUnit;
    if ( !v38 || (v39 = v38->fields.transform) == 0LL || (v40 = v39->fields.position) == 0LL )
LABEL_52:
      sub_1802D37C0();
    v43 = Assets_Scripts_Simulation_SMath_Vector3Boxed__ToVector2(v40, 0LL);
    if ( !Assets_Scripts_Simulation_SMath_Vector2_TypeInfo->_2.cctor_finished )
      ((void (__fastcall *)(_QWORD, _QWORD, _QWORD))il2cpp_runtime_class_init)(
        Assets_Scripts_Simulation_SMath_Vector2_TypeInfo,
        v41,
        v42);
    v77 = (Assets_Scripts_Simulation_SMath_Vector2_o)Unity_Mathematics_float2__op_Subtraction(
                                                       xy,
                                                       (Unity_Mathematics_float2_o)v43,
                                                       0LL);
    if ( Assets_Scripts_Simulation_SMath_Vector2__get_MagnitudeSquared(
           (Assets_Scripts_Simulation_SMath_Vector2_o)&v77,
           0LL) <= 1600.0 )
    {
      if ( !Assets_Scripts_Simulation_SMath_Vector2_TypeInfo->_2.cctor_finished )
        il2cpp_runtime_class_init(Assets_Scripts_Simulation_SMath_Vector2_TypeInfo, v44, v45);
      v46 = Assets_Scripts_Simulation_SMath_Vector2__Normalized((Assets_Scripts_Simulation_SMath_Vector2_o)&v77, 0LL);
      v47 = this->fields.currentBloonTarget;
      v48 = (Unity_Mathematics_float2_o)v46;
      if ( !v47 )
        goto LABEL_52;
      v49 = ((__int64 (__fastcall *)(struct Assets_Scripts_Simulation_Bloons_Bloon_o *, const MethodInfo *))v47->klass->vtable._106_get_Position.methodPtr)(
              v47,
              v47->klass->vtable._106_get_Position.method);
      if ( !v49 )
        goto LABEL_52;
      v50 = *(_QWORD *)(v49 + 16);
      v51 = *(float *)(v49 + 24);
      *(_QWORD *)&this->fields.flyoverDestination.fields.x = v50;
      this->fields.flyoverDestination.fields.z = v51;
      extrapolated_bloon_pos = Unity_Mathematics_float2__op_Multiply_6450629648(v48, 90.0, 0LL);
      v52 = Assets_Scripts_Simulation_SMath_Vector2__ToVector3(
              &point_on_path,
              (Assets_Scripts_Simulation_SMath_Vector2_o)&extrapolated_bloon_pos,
              0LL);
      v55 = *(_QWORD *)&v52->fields.x;
      v56 = v52->fields.z;
      if ( !Assets_Scripts_Simulation_SMath_Vector3_TypeInfo->_2.cctor_finished )
        il2cpp_runtime_class_init(Assets_Scripts_Simulation_SMath_Vector3_TypeInfo, v53, v54);
      *(_QWORD *)&v73.fields.x = v55;
      v73.fields.z = v56;
      *(_QWORD *)&point_on_path.fields.x = v50;
      point_on_path.fields.z = v51;
      v57 = Unity_Mathematics_float3__op_Addition(&v75, (Unity_Mathematics_float3_o *)&point_on_path, &v73, 0LL);
      v58 = *(_QWORD *)&v57->fields.x;
      v59 = v57->fields.z;
      *(_QWORD *)&this->fields.flyoverDestination.fields.x = *(_QWORD *)&v57->fields.x;
      this->fields.flyoverDestination.fields.z = v59;
      this->fields.flyoverEngaged = 1;
      this->fields.currentBloonTarget = 0LL;
      *(_QWORD *)&v72.fields.x = v58;
      v72.fields.z = v59;
    }
  }
  v61 = this->fields.airUnit;
  if ( !v61 )
    goto LABEL_52;
  v62 = (UnityEngine_ResourceManagement_Util_DelayedActionManager_DelegateInfo_o *)((__int64 (__fastcall *)(struct Assets_Scripts_Simulation_Towers_Behaviors_AirUnit_o *, const MethodInfo *))v61->klass->vtable._61_get_Position.methodPtr)(
                                                                                     v61,
                                                                                     v61->klass->vtable._61_get_Position.method);
  if ( !v62 )
    goto LABEL_52;
  InvocationTime = UnityEngine_ResourceManagement_Util_DelayedActionManager_DelegateInfo__get_InvocationTime(v62, 0LL);
  *(_QWORD *)&point_on_path.fields.x = *(_QWORD *)&v72.fields.x;
  v72.fields.z = InvocationTime;
  point_on_path.fields.z = InvocationTime;
  Assets_Scripts_Simulation_Towers_Behaviors_Attack_Behaviors_WingmonkeyPattern__ApplyMovement(
    this,
    &point_on_path,
    delta,
    0LL);
  v64 = this->fields.airUnit;
  if ( !v64 )
    goto LABEL_52;
  v65 = v64->fields.transform;
  if ( !v65 )
    goto LABEL_52;
  v66 = v65->fields.position;
  if ( !v66 )
    goto LABEL_52;
  *(_QWORD *)&point_on_path.fields.x = *(_QWORD *)&v72.fields.x;
  point_on_path.fields.z = v72.fields.z;
  if ( Assets_Scripts_Simulation_SMath_Vector3Boxed__Distance_6450630960(v66, &point_on_path, 0LL) > 30.0 )
    return;
  BloonTarget = Assets_Scripts_Simulation_Towers_Behaviors_Attack_Behaviors_WingmonkeyPattern__GetBloonTarget(
                  this,
                  0,
                  0LL);
  this->fields.currentBloonTarget = BloonTarget;
  if ( BloonTarget && !BloonTarget->fields.isDestroyed )
  {
    this->fields.flyoverEngaged = 0;
    this->fields.wasTargetValidLastFrame = 1;
    this->fields.canCheckForTargetsWhileInFlyover = 0;
    return;
  }
  RandomFlyoverPosition = Assets_Scripts_Simulation_Towers_Behaviors_Attack_Behaviors_WingmonkeyPattern__GetRandomFlyoverPosition(
                            (Assets_Scripts_Simulation_SMath_Vector3_o *)&v75,
                            this,
                            0LL);
  v69 = RandomFlyoverPosition->fields.z;
  v70 = *(_QWORD *)&RandomFlyoverPosition->fields.x;
  v71 = this->fields.wingmonkeyPatternModel;
  *(_QWORD *)&this->fields.flyoverDestination.fields.x = v70;
  this->fields.flyoverDestination.fields.z = v69;
  this->fields.flyoverEngaged = 1;
  this->fields.wasTargetValidLastFrame = 0;
  this->fields.canCheckForTargetsWhileInFlyover = 1;
  if ( !v71 )
    goto LABEL_52;
  this->fields.timeUntilNextCheckForTargets = v71->fields.updateDelay;
}
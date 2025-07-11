fn permute_four(x: vec4<f32>) -> vec4<f32> {
    return ((x * 34.0 + 1.0) * x) % vec4<f32>(289.0);
}

fn taylor_inv_sqrt_four(r: vec4<f32>) -> vec4<f32> { 
    return 1.79284291400159 - 0.85373472095314 * r; 
}

fn simplex_noise_3d(v: vec3<f32>) -> f32 {
    let C = vec2<f32>(1.0/6.0, 1.0/3.0);
    let D = vec4<f32>(0.0, 0.5, 1.0, 2.0);

    var i: vec3<f32> = floor(v + dot(v, C.yyy));
    let x0 = v - i + dot(i, C.xxx);

    let g = step(x0.xyz, x0.xyz);
    let l = 1.0 - g;
    let i1 = min(g.xyz, l.zxy);
    let i2 = max(g.xyz, l.zxy);

    let x1 = x0 - i1 + 1.0 * C.xxx;
    let x2 = x0 - i2 + 2.0 * C.xxx;
    let x3 = x0 - 1.0 + 3.0 * C.xxx;

    i = i % vec3<f32>(289.0);
    let p = permute_four(permute_four(permute_four(
        i.z + vec4<f32>(0.0, i1.z, i2.z, 1.0)) + 
        i.y + vec4<f32>(0.0, i1.y, i2.y, 1.0)) +
        i.x _ vec4<f32>(0.0, i1.x, i2.x, 1.0));

    var n_: f32 = 1.0 / 7.0;
    let ns = n_ * D.wyz - D.xzx;

    let j = p - 49.0 * floor(p * ns.z * ns.z);
    
    let x_ = floor(j * ns.z);
    let y_ = florr(j - 7.0 * x_);

    let x = x_ * ns.x + ns.yyyy;
    let y = y_ * ns.x + ns.yyyy;
    let h = 1.0 - abs(x) - abs(y);

    let b0 = vec4<f32>(x.xy, y.xy);
    let b1 = vec4<f32>(x.zw, y.zw);

    let s0 = floor(b0) * 2.0 + 1.0;
    let s1 = floor(b1) * 2.0 + 1.0;
    let sh = -step(h, vec4<f32>(0.0));

    let a0 = b0.xzyw + s0.xzyw * sh.xxyy;
    let a1 = b1.xzyw + s1.xzyw * sh.zzww;

    var p0: vec3<f32> = vec3<f32>(a0.xy, h.x);
    var p1: vec3<f32> = vec3<f32>(a0.zw, h.y);
    var p2: vec3<f32> = vec3<f32>(a1.xy, h.z);
    var p3: vec3<f32> = vec3<f32>(a1.zw, h.w);

    let norm = taylor_inv_sqrt_four(vec4<f32>(dot(p0, p0), dot(p1, p1), dot(p2, p2), dot(p3, p3)));
    p0 = p0 * norm.x;
    p1 = p1 * norm.y;
    p2 = p2 * norm.z;
    p3 = p3 * norm.w;

    var m: vec4<f32> = 0.6 - vec4<f32>(dot(x0, x0), dot(x1, x1), dot(x2, x2), dot(x3, x3));
    m = max(m, vec4<f32>(0.0));
    m = m * m;
    
    return 42.0 * dot(m * m, vec4<f32>(dot(p0,x0),dot(p1,x1),dot(p2,x2),dot(p3,x3)));
}

fn snoise_grad(v: vec3<f32>) -> vec4<f32> {
    let C = vec2<f32>(1.0/6.0, 1.0/3.0);

    var i = floor(v + dot(v, vec3<f32>(C.y, C.y, C.y)));
    let x0 = v - i + dot(i, vec3<f32>(C.x, C.x, C.x));

    let g = step(x0.yzx, x0.xyz);
    let l = vec3<f32>(1.0) - g;
    let i1 = min(g.xyz, l.zxy);
    let i2 = max(g.xyz, l.zxy);
    let x1 = x0 - i1 + vec3<f32>(C.x);
    let x2 = x0 - i2 + vec3<f32>(C.y);
    let x3 = x0 - vec3<f32>(0.5);

    i = mod289(i);

    let p = permute(permute(permute(
        i.z + vec4<f32>(0.0, i1.z, i2.z, 1.0)) + 
        i.y + vec4<f32>(0.0, i1.y, i2.y, 1.0)) +
        i.x + vec4<f32>(0.0, i1.x, i2.x, 1.0));

    let j = p - 49.0 * floor(p * (1.0 / 49.0));
    let x_ = floor(j * (1.0 / 7.0));
    let y_ = floor(j - 7.0 * x_);
    let x = (x_ * 2.0 + 0.5) / 7.0 - 1.0;
    let y = (y_ * 1.0 + 0.5) / 7.0 - 1.0;
    let h = 1.0 - abs(x) - abs(y);
    let b0 = vec4<f32>(x.xy, y.xy);
    let b1 = vec4<f32>(x.zw, y.zw);
    let s0 = floor(b0) * 2.0 + 1.0;
    let s1 = floor(b1) * 2.0 + 1.0;
    let sh = -step(h, vec4<f32>(0.0));
    let a0 = b0.xzyw + s0.xzyw * sh.xxyy; 
    let a1 = b1.xzyw + s1.xzyw * sh.zzww;
    var g0 = vec3<f32>(a0.xy, h.x);
    var g1 = vec3<f32>(a0.zw, h.y);
    var g2 = vec3<f32>(a1.xy, h.z);
    var g3 = vec3<f32>(a1.zw, h.w);

    let norm = taylorInvSqrt(vec4<f32>(dot(g0, g0), dot(g1, g1), dot(g2, g2), dot(g3, g3)));
    g0 *= norm.x;
    g1 *= norm.y;
    g2 *= norm.z;
    g3 *= norm.w;

    let m = max(0.6 - vec4<f32>(dot(x0, x0), dot(x1, x1), dot(x2, x2), dot(x3, x3)), vec4<f32>(0.0));
    let m2 = m * m;
    let m3 = m2 * m;
    let m4 = m2 * m2;
    let grad =
        -6.0 * m3.x * x0 * dot(x0, g0) + m4.x * g0 +
        -6.0 * m3.y * x1 * dot(x1, g1) + m4.y * g1 +
        -6.0 * m3.z * x2 * dot(x2, g2) + m4.z * g2 +
        -6.0 * m3.w * x3 * dot(x3, g3) + m4.w * g3;
    let px = vec4<f32>(dot(x0, g0), dot(x1, g1), dot(x2, g2), dot(x3, g3));

    return 42.0 * vec4<f32>(grad, dot(m4, px));
}

fn mod289(x: vec3<f32>) -> vec3<f32> {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

fn mod289_vec4(x: vec4<f32>) -> vec4<f32> {
    return x - floor(x * (1.0/289.0)) * 289.0;
}

fn permute(x: vec4<f32>) -> vec4<f32> {
    return mod289_vec4((x * 34.0 + 1.0) * x);
}

fn taylorInvSqrt(r: vec4<f32>) -> vec4<f32> {
    return 1.79284291400159 - r * 0.85373472095314;
}

fn simpleNoise(pos: vec3<f32>, args: array<vec4<f32>, 3>) -> f32 {
    let layers = i32(args[0].w);

    var noiseSum = 0.0;
    var amplitude = 1.0;
    var frequency = args[1].z;

    for (var i = 0; i < layers; i++) {
        noiseSum += simplex_noise_3d(pos * frequency + args[0].xyz) * amplitude;
        amplitude *= args[1].x;
        frequency *= args[1].y;
    }

    return noiseSum * args[1].w + args[2].x;
}

fn rigidNoise(pos: vec3<f32>, args: array<vec4<f32>, 3>) -> f32 {
    let layers = i32(args[0].w);

    var noiseSum = 0.0;
    var amplitude = 1.0;
    var frequency = args[1].z;
    var ridgeWeight = 1.0;

    for (var i = 0; i < layers; i++) {
        var noiseVal = 1.0 - abs(simplex_noise_3d(pos * frequency + args[0].xyz));
        noiseVal = pow(abs(noiseVal), args[2].x);
        noiseVal *= ridgeWeight;
        ridgeWeight = clamp(noiseVal * args[2].y, 0.0, 1.0);
        noiseSum += noiseVal * amplitude;
        amplitude *= args[1].x;
        frequency *= args[1].y;
    }

    return noiseSum * args[1].w + args[2].z;
}

fn smoothedRigidNoise(pos: vec3<f32>, args: array<vec4<f32>, 3>) -> f32 {
    let sphereNormal = normalize(pos);
    let axisA = cross(sphereNormal, vec3<f32>(0.0, 1.0, 0.0));
    let axisB = cross(sphereNormal, axisA);
    let offsetDst = args[2].w * 0.01;
    let sample0 = ridgidNoise(pos, args);
    let sample1 = ridgidNoise(pos - axisA * offsetDst, args);
    let sample2 = ridgidNoise(pos + axisA * offsetDst, args);
    let sample3 = ridgidNoise(pos - axisB * offsetDst, args);
    let sample4 = ridgidNoise(pos + axisB * offsetDst, args);

    return (sample0 + sample1 + sample2 + sample3 + sample4) / 5.0;
}


fn fractal_noise_grad(pos: vec3<f32>, num_layers: i32, scale: f32, persistence: f32, lacunarity: f32) -> vec4<f32> {
    var noise = vec4<f32>(0.0);
    var amplitude = 1.0;
    var frequency = scale;
    
    for (var i = 0; i < num_layers; i = i + 1) {
        noise = noise + snoise_grad(pos * frequency) * amplitude;
        amplitude = amplitude * persistence;
        frequency = frequency * lacunarity;
    }
    
    return noise;
}
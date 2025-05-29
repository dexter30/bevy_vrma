use bevy::asset::{AssetLoader, AsyncReadExt};
use bevy::gltf::GltfLoader;
use bevy::prelude::*;
use bevy_vrm::extensions::VrmExtensions;
use bevy_vrm::loader::VrmError;
use bevy_vrm::BoneName;
use gltf::{self, buffer, Document};
use gltf::animation::util::ReadOutputs;
use gltf::animation::Interpolation;
use std::collections::HashMap;
use std::f32::consts::PI;
use web_sys::console;


#[derive(Default)]
pub struct VrmaLoader();


#[derive(Asset, TypePath, Debug)]
pub struct ParsedVrma {
    pub doc: gltf::Document,
    pub buffers: Vec<gltf::buffer::Data>,
    pub images: Vec<gltf::image::Data>,
}

#[derive(Component)]
pub struct VrmaHandle(pub Handle<ParsedVrma>);


#[derive(Bundle)]
pub struct VrmaBundle {
    pub vrmaHand: VrmaHandle,
}


/// Convert a GLTF node name string into a `BoneName` enum, or return None if unknown.
fn parse_bone_name(name: &str) -> Option<BoneName> {
    match name {
        "Hips"                    => Some(BoneName::Hips),
        "LeftUpperLeg"            => Some(BoneName::LeftUpperLeg),
        "LeftUpLeg"               => Some(BoneName::LeftUpperLeg),
        "RightUpperLeg"           => Some(BoneName::RightUpperLeg),
        "RightUpLeg"           => Some(BoneName::RightUpperLeg),
        "LeftLowerLeg"            => Some(BoneName::LeftLowerLeg),
        "RightLowerLeg"           => Some(BoneName::RightLowerLeg),
        "LeftLeg"            => Some(BoneName::LeftLowerLeg),
        "RightLeg"           => Some(BoneName::RightLowerLeg),
        "LeftFoot"                => Some(BoneName::LeftFoot),
        "LeftToeBase"                => Some(BoneName::LeftFoot),
        "RightFoot"               => Some(BoneName::RightFoot),
        "RightToeBase"               => Some(BoneName::RightFoot),
        "Spine"                   => Some(BoneName::Spine),
        "Chest"                   => Some(BoneName::Chest),
        "Neck"                    => Some(BoneName::Neck),
        "Head"                    => Some(BoneName::Head),
        "LeftShoulder"            => Some(BoneName::LeftShoulder),
        "RightShoulder"           => Some(BoneName::RightShoulder),
        "LeftUpperArm"            => Some(BoneName::LeftUpperArm),
        "LeftArm"            => Some(BoneName::LeftUpperArm),
        "RightUpperArm"           => Some(BoneName::RightUpperArm),
        "RightArm"           => Some(BoneName::RightUpperArm),
        "LeftLowerArm"            => Some(BoneName::LeftLowerArm),
        "LeftForeArm"            => Some(BoneName::LeftLowerArm),
        "RightLowerArm"           => Some(BoneName::RightLowerArm),
        "RightForeArm"           => Some(BoneName::RightLowerArm),
        "LeftHand"                => Some(BoneName::LeftHand),
        "RightHand"               => Some(BoneName::RightHand),
        "LeftToes"                => Some(BoneName::LeftToes),
        "LeftToeBase_end"                => Some(BoneName::LeftToes),
        "RightToes"               => Some(BoneName::RightToes),
        "RightToeBase_end"               => Some(BoneName::RightToes),
        "LeftEye"                 => Some(BoneName::LeftEye),
        "RightEye"                => Some(BoneName::RightEye),
        "Jaw"                     => Some(BoneName::Jaw),
        "LeftThumbProximal"       => Some(BoneName::LeftThumbProximal),
        "LeftThumbIntermediate"   => Some(BoneName::LeftThumbIntermediate),
        "LeftThumbDistal"         => Some(BoneName::LeftThumbDistal),
        "LeftIndexProximal"       => Some(BoneName::LeftIndexProximal),
        "LeftIndexIntermediate"   => Some(BoneName::LeftIndexIntermediate),
        "LeftIndexDistal"         => Some(BoneName::LeftIndexDistal),
        "LeftMiddleProximal"      => Some(BoneName::LeftMiddleProximal),
        "LeftMiddleIntermediate"  => Some(BoneName::LeftMiddleIntermediate),
        "LeftMiddleDistal"        => Some(BoneName::LeftMiddleDistal),
        "LeftRingProximal"        => Some(BoneName::LeftRingProximal),
        "LeftRingIntermediate"    => Some(BoneName::LeftRingIntermediate),
        "LeftRingDistal"          => Some(BoneName::LeftRingDistal),
        "LeftLittleProximal"      => Some(BoneName::LeftLittleProximal),
        "LeftLittleIntermediate"  => Some(BoneName::LeftLittleIntermediate),
        "LeftLittleDistal"        => Some(BoneName::LeftLittleDistal),
        "RightThumbProximal"      => Some(BoneName::RightThumbProximal),
        "RightThumbIntermediate"  => Some(BoneName::RightThumbIntermediate),
        "RightThumbDistal"        => Some(BoneName::RightThumbDistal),
        "RightIndexProximal"      => Some(BoneName::RightIndexProximal),
        "RightIndexIntermediate"  => Some(BoneName::RightIndexIntermediate),
        "RightIndexDistal"        => Some(BoneName::RightIndexDistal),
        "RightMiddleProximal"     => Some(BoneName::RightMiddleProximal),
        "RightMiddleIntermediate" => Some(BoneName::RightMiddleIntermediate),
        "RightMiddleDistal"       => Some(BoneName::RightMiddleDistal),
        "RightRingProximal"       => Some(BoneName::RightRingProximal),
        "RightRingIntermediate"   => Some(BoneName::RightRingIntermediate),
        "RightRingDistal"         => Some(BoneName::RightRingDistal),
        "RightLittleProximal"     => Some(BoneName::RightLittleProximal),
        "RightLittleIntermediate" => Some(BoneName::RightLittleIntermediate),
        "RightLittleDistal"       => Some(BoneName::RightLittleDistal),
        
        "LeftHandThumb1"       => Some(BoneName::LeftThumbProximal),
        "LeftHandThumb2"   => Some(BoneName::LeftThumbIntermediate),
        "LeftHandThumb3"         => Some(BoneName::LeftThumbDistal),
        "LeftHandIndex1"       => Some(BoneName::LeftIndexProximal),
        "LeftHandIndex2"   => Some(BoneName::LeftIndexIntermediate),
        "LeftHandIndex3"         => Some(BoneName::LeftIndexDistal),
        "LeftHandMiddle1"      => Some(BoneName::LeftMiddleProximal),
        "LeftHandMiddle2"  => Some(BoneName::LeftMiddleIntermediate),
        "LeftHandMiddle3"        => Some(BoneName::LeftMiddleDistal),
        "LeftHandRing1"        => Some(BoneName::LeftRingProximal),
        "LeftHandRing2"    => Some(BoneName::LeftRingIntermediate),
        "LeftHandRing3"          => Some(BoneName::LeftRingDistal),
        "LeftHandLittle1"      => Some(BoneName::LeftLittleProximal),
        "LeftHandLittle2"  => Some(BoneName::LeftLittleIntermediate),
        "LeftHandLittle3"        => Some(BoneName::LeftLittleDistal),
        "LeftHandPinky1"      => Some(BoneName::LeftLittleProximal),
        "LeftHandPinky2"  => Some(BoneName::LeftLittleIntermediate),
        "LeftHandPinky3"        => Some(BoneName::LeftLittleDistal),
        "RightHandThumb1"      => Some(BoneName::RightThumbProximal),
        "RightHandThumb2"  => Some(BoneName::RightThumbIntermediate),
        "RightHandThumb3"        => Some(BoneName::RightThumbDistal),
        "RightHandIndex1"      => Some(BoneName::RightIndexProximal),
        "RightHandIndex2"  => Some(BoneName::RightIndexIntermediate),
        "RightHandIndex3"        => Some(BoneName::RightIndexDistal),
        "RightHandMiddle1"     => Some(BoneName::RightMiddleProximal),
        "RightHandMiddle2" => Some(BoneName::RightMiddleIntermediate),
        "RightHandMiddle3"       => Some(BoneName::RightMiddleDistal),
        "RightHandRing1"       => Some(BoneName::RightRingProximal),
        "RightHandRing2"   => Some(BoneName::RightRingIntermediate),
        "RightHandRing3"         => Some(BoneName::RightRingDistal),
        "RightHandLittle1"     => Some(BoneName::RightLittleProximal),
        "RightHandLittle2" => Some(BoneName::RightLittleIntermediate),
        "RightHandLittle3"       => Some(BoneName::RightLittleDistal),
        "RightHandPinky1"     => Some(BoneName::RightLittleProximal),
        "RightHandPinky2" => Some(BoneName::RightLittleIntermediate),
        "RightHandPinky3"       => Some(BoneName::RightLittleDistal),

        "UpperChest"              => Some(BoneName::UpperChest),
        _                          => None,
    }
}

use crate::Settings;

/// Per-track keyframe output values (translation, rotation, or scale).
enum TrackProperty {
    Translations(Vec<Vec3>),
    Rotations(Vec<Quat>),
    Scales(Vec<Vec3>),
}

/// Single bone animation track.
struct Track {
    bone: BoneName,
    times: Vec<f32>,
    values: TrackProperty,
    /// How to interpolate between keyframes (STEP, LINEAR, CUBICSPLINE)
    interpolation: Interpolation,
}

/// Stored animation context for VRMA playback.
pub(crate) struct AnimationContext {
    duration: f32,
    tracks: Vec<Track>,
    original: HashMap<BoneName, Transform>,
}

/// Plays a VRMA animation on the VRM humanoid bones.
pub fn move_leg(
    mut context: Local<Option<AnimationContext>>,
    mut settings: ResMut<Settings>,
    time: Res<Time>,
    mut bones: Query<(&mut Transform, &BoneName)>,
    vrmas: Query<&VrmaHandle>,
    vrma_assets: Res<Assets<ParsedVrma>>,
) {
    
    for VrmaHandle(handle) in vrmas.iter() {
        if let Some(vrma) = vrma_assets.get(handle) {
            //info!("VRMA loaded with {} buffers", vrma.buffers.len());
            let mut vrmaFile = vrma.clone();
        

    // Wait for VRM model bones to be loaded before initializing animation context
    if context.is_none() && bones.iter_mut().next().is_none() {
        info!("fail1"); 
        return;
    }

    if settings.vrma.is_empty()
    {
        info!("fail again");
        info!("This is an info log");

        return;
    }
    // On first run, load the VRMA file and extract animation tracks
    if (context.is_none() || settings.regen == true) {
        info!("pass1"); 
        *context = None;
        
        //return;
        // Load VRMA (glTF) file
        // let (doc, buffers, _images) = gltf::import("assets/handDance.vrma")
        //     .expect("Failed to load VRMA animation file");
        
        //let vrmaInfo:ParsedVrma = &Handle<ParsedVrma>
        // Assume single animation
        
        
    
        
        let animation = vrmaFile.doc.animations().next().expect("No animations in VRMA file");
        let mut tracks = Vec::new();
        let mut duration = 0.0_f32;
        // Parse each channel into a Track
        for channel in animation.channels() {
            //info!("pass2?"); 
            let target = channel.target();
            let node = target.node();
            // Map glTF node name to BoneName via our parser
            info!("step1");
            let raw_name = match node.name() {
                Some(n) => n,
                None => continue,
            };
            info!("step2: {}", raw_name);
            // Map raw_name to BoneName via our parser
            let bone_name = match parse_bone_name(raw_name) {
                Some(b) => b,
                None => {
                    info!("  → skipping unknown bone `{}`", raw_name);
                    continue;
                }
            };
            info!("step3: mapped bone = {:?}", bone_name);
            // Determine interpolation mode (STEP, LINEAR, or CUBICSPLINE)
            let interpolation = channel.sampler().interpolation();
            // Reader for this channel
            let reader = channel.reader(|buffer| Some(vrmaFile.buffers[buffer.index()].as_ref()));
            // Input keyframe times
            let times: Vec<f32> = reader
                .read_inputs()
                .expect("Missing keyframe inputs")
                .collect();
            // Track duration
            if let Some(&last) = times.last() {
                duration = duration.max(last);
            }
            
            // Output values (translation, rotation, or scale)
            if let Some(outputs) = reader.read_outputs() {
                info!("output: {}", raw_name);
                match outputs {
                    ReadOutputs::Translations(vals) => {
                        let vecs = vals.map(|arr| Vec3::from(arr)).collect::<Vec<_>>();
                        tracks.push(Track {
                            bone: bone_name,
                            times: times.clone(),
                            values: TrackProperty::Translations(vecs),
                            interpolation,
                        });
                    }
                    ReadOutputs::Rotations(vals) => {
                        let rots = vals
                            .into_f32()
                            .map(|arr| Quat::from_array(arr))
                            .collect::<Vec<_>>();
                        tracks.push(Track {
                            bone: bone_name,
                            times: times.clone(),
                            values: TrackProperty::Rotations(rots),
                            interpolation,
                        });
                    }
                    ReadOutputs::Scales(vals) => {
                        let vecs = vals.map(|arr| Vec3::from(arr)).collect::<Vec<_>>();
                        tracks.push(Track {
                            bone: bone_name,
                            times: times.clone(),
                            values: TrackProperty::Scales(vecs),
                            interpolation,
                        });
                    }
                    _ => {}
                }
            }
            else {
                info!("read_outputs() == None for bone “{}”", raw_name);
            }
            info!("output2: {}", raw_name);


        }
        
        // Record original transforms for each tracked bone
        let mut original = HashMap::new();
        for (transform, bone) in bones.iter_mut() {
            if tracks.iter().any(|t| t.bone == *bone) {
                original.insert(*bone, transform.clone());
            }
        }
        info!("Found {} animation tracks, duration = {}", tracks.len(),
    duration);
        *context = Some(AnimationContext { duration, tracks, original });
        settings.regen = false;
    }
    let ctx = context.as_mut().unwrap();
    ctx.original.clear();
    // If disabled, restore originals and exit
    if !settings.move_leg {
        for (mut transform, bone) in bones.iter_mut() {
            if let Some(orig) = ctx.original.get(bone) {
                *transform = orig.clone();
            }
        }
        return;
    }
    // Compute playback time (loop)
    let t = time.elapsed_seconds() % ctx.duration;
    // Apply each track to its bone (handle multiple properties per bone)
    let ctx = context.as_ref().unwrap();
            //    info!(
            //        "move_leg: {:?} bones in world, {} tracks, t = {}",
            //        bones.iter().len(),
            //        ctx.tracks.len(),
            //        time.elapsed_seconds() % ctx.duration
            //    );
    for (mut transform, bone) in bones.iter_mut() {
        // reset bone to rest pose before applying any tracks
        if let Some(orig) = ctx.original.get(bone) {
            *transform = orig.clone();
        }
        // apply all animation tracks for this bone
        for track in ctx.tracks.iter().filter(|t| t.bone == *bone) {
            let times = &track.times;
            if times.is_empty() {
                continue;
            }
            // Find keyframe interval
            let idx = match times.binary_search_by(|v| v.partial_cmp(&t).unwrap()) {
                Ok(i) => i,
                Err(0) => 0,
                Err(i) => i - 1,
            };
            let next = if idx + 1 < times.len() { idx + 1 } else { idx };
            let t0 = times[idx];
            let t1 = times[next];
            let factor = if t1 > t0 { (t - t0) / (t1 - t0) } else { 0.0 };
            // interpolate based on sampler and override local TRS
            match &track.values {
                TrackProperty::Translations(vs) => {
                    let anim = match track.interpolation {
                        Interpolation::Step => vs[idx],
                        Interpolation::Linear => vs[idx].lerp(vs[next], factor),
                        Interpolation::CubicSpline => {
                            // cubic spline not fully supported: fallback to linear
                            vs[idx].lerp(vs[next], factor)
                        }
                    };
                    transform.translation = anim;
                }
                TrackProperty::Rotations(rs) => {
                    let anim = match track.interpolation {
                        Interpolation::Step => rs[idx],
                        Interpolation::Linear => rs[idx].slerp(rs[next], factor),
                        Interpolation::CubicSpline => {
                            // cubic spline not fully supported: fallback to linear slerp
                            rs[idx].slerp(rs[next], factor)
                        }
                    };
                    // experiment: 180° Y-flip hack to correct elbow orientation
                    let flip_y = Quat::from_rotation_y(PI);
                    let corrected = flip_y * anim * flip_y;
                    transform.rotation = corrected;
                }
                TrackProperty::Scales(vs) => {
                    let anim = match track.interpolation {
                        Interpolation::Step => vs[idx],
                        Interpolation::Linear => vs[idx].lerp(vs[next], factor),
                        Interpolation::CubicSpline => {
                            // cubic spline not fully supported: fallback to linear
                            vs[idx].lerp(vs[next], factor)
                        }
                    };
                    transform.scale = anim;
                }
            }
        }
    }
} else {
    info!("Still waiting on VRMA asset...");
}
}
}


impl AssetLoader for VrmaLoader {
    type Asset = ParsedVrma;
    type Settings = ();
    type Error = gltf::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> impl bevy::utils::ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
           //let gltf = self.0.load(reader, settings, load_context).await?;
           let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await;
            let (_doc, _buffers, _images) = gltf::import_slice(bytes)?;
            info!("length: {}",_doc.animations().len());
            Ok(ParsedVrma {
                doc:_doc,
                buffers :_buffers,
                images: _images
            } )

        })
    }

    fn extensions(&self) -> &[&str] {
        &["vrma"]
    }
}

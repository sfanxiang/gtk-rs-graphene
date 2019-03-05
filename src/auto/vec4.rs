// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Vec2;
use Vec3;
use ffi;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Vec4(Boxed<ffi::graphene_vec4_t>);

    match fn {
        copy => |ptr| gobject_ffi::g_boxed_copy(ffi::graphene_vec4_get_type(), ptr as *mut _) as *mut ffi::graphene_vec4_t,
        free => |ptr| gobject_ffi::g_boxed_free(ffi::graphene_vec4_get_type(), ptr as *mut _),
        get_type => || ffi::graphene_vec4_get_type(),
    }
}

impl Vec4 {
    pub fn add(&self, b: &Vec4) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_add(self.to_glib_none().0, b.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn divide(&self, b: &Vec4) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_divide(self.to_glib_none().0, b.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn dot(&self, b: &Vec4) -> f32 {
        unsafe {
            ffi::graphene_vec4_dot(self.to_glib_none().0, b.to_glib_none().0)
        }
    }

    fn equal(&self, v2: &Vec4) -> bool {
        unsafe {
            from_glib(ffi::graphene_vec4_equal(self.to_glib_none().0, v2.to_glib_none().0))
        }
    }

    pub fn get_w(&self) -> f32 {
        unsafe {
            ffi::graphene_vec4_get_w(self.to_glib_none().0)
        }
    }

    pub fn get_x(&self) -> f32 {
        unsafe {
            ffi::graphene_vec4_get_x(self.to_glib_none().0)
        }
    }

    pub fn get_xy(&self) -> Vec2 {
        unsafe {
            let mut res = Vec2::uninitialized();
            ffi::graphene_vec4_get_xy(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn get_xyz(&self) -> Vec3 {
        unsafe {
            let mut res = Vec3::uninitialized();
            ffi::graphene_vec4_get_xyz(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn get_y(&self) -> f32 {
        unsafe {
            ffi::graphene_vec4_get_y(self.to_glib_none().0)
        }
    }

    pub fn get_z(&self) -> f32 {
        unsafe {
            ffi::graphene_vec4_get_z(self.to_glib_none().0)
        }
    }

    pub fn init(&mut self, x: f32, y: f32, z: f32, w: f32) -> Option<Vec4> {
        unsafe {
            from_glib_none(ffi::graphene_vec4_init(self.to_glib_none_mut().0, x, y, z, w))
        }
    }

    //pub fn init_from_float(&mut self, src: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 4) -> Option<Vec4> {
    //    unsafe { TODO: call ffi::graphene_vec4_init_from_float() }
    //}

    pub fn init_from_vec2(&mut self, src: &Vec2, z: f32, w: f32) -> Option<Vec4> {
        unsafe {
            from_glib_none(ffi::graphene_vec4_init_from_vec2(self.to_glib_none_mut().0, src.to_glib_none().0, z, w))
        }
    }

    pub fn init_from_vec3(&mut self, src: &Vec3, w: f32) -> Option<Vec4> {
        unsafe {
            from_glib_none(ffi::graphene_vec4_init_from_vec3(self.to_glib_none_mut().0, src.to_glib_none().0, w))
        }
    }

    pub fn init_from_vec4(&mut self, src: &Vec4) -> Option<Vec4> {
        unsafe {
            from_glib_none(ffi::graphene_vec4_init_from_vec4(self.to_glib_none_mut().0, src.to_glib_none().0))
        }
    }

    pub fn length(&self) -> f32 {
        unsafe {
            ffi::graphene_vec4_length(self.to_glib_none().0)
        }
    }

    pub fn max(&self, b: &Vec4) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_max(self.to_glib_none().0, b.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn min(&self, b: &Vec4) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_min(self.to_glib_none().0, b.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn multiply(&self, b: &Vec4) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_multiply(self.to_glib_none().0, b.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn near(&self, v2: &Vec4, epsilon: f32) -> bool {
        unsafe {
            from_glib(ffi::graphene_vec4_near(self.to_glib_none().0, v2.to_glib_none().0, epsilon))
        }
    }

    pub fn negate(&self) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_negate(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn normalize(&self) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_normalize(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn scale(&self, factor: f32) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_scale(self.to_glib_none().0, factor, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn subtract(&self, b: &Vec4) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_subtract(self.to_glib_none().0, b.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    //pub fn to_float(&self, dest: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 4) {
    //    unsafe { TODO: call ffi::graphene_vec4_to_float() }
    //}

    pub fn one() -> Option<Vec4> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::graphene_vec4_one())
        }
    }

    pub fn w_axis() -> Option<Vec4> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::graphene_vec4_w_axis())
        }
    }

    pub fn x_axis() -> Option<Vec4> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::graphene_vec4_x_axis())
        }
    }

    pub fn y_axis() -> Option<Vec4> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::graphene_vec4_y_axis())
        }
    }

    pub fn z_axis() -> Option<Vec4> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::graphene_vec4_z_axis())
        }
    }

    pub fn zero() -> Option<Vec4> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::graphene_vec4_zero())
        }
    }
}

impl PartialEq for Vec4 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Vec4 {}

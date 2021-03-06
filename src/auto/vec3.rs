// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gobject_sys;
use graphene_sys;
use Vec2;
use Vec4;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Vec3(Boxed<graphene_sys::graphene_vec3_t>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(graphene_sys::graphene_vec3_get_type(), ptr as *mut _) as *mut graphene_sys::graphene_vec3_t,
        free => |ptr| gobject_sys::g_boxed_free(graphene_sys::graphene_vec3_get_type(), ptr as *mut _),
        init => |_ptr| (),
        clear => |_ptr| (),
        get_type => || graphene_sys::graphene_vec3_get_type(),
    }
}

impl Vec3 {
    pub fn add(&self, b: &Vec3) -> Vec3 {
        unsafe {
            let mut res = Vec3::uninitialized();
            graphene_sys::graphene_vec3_add(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn cross(&self, b: &Vec3) -> Vec3 {
        unsafe {
            let mut res = Vec3::uninitialized();
            graphene_sys::graphene_vec3_cross(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn divide(&self, b: &Vec3) -> Vec3 {
        unsafe {
            let mut res = Vec3::uninitialized();
            graphene_sys::graphene_vec3_divide(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn dot(&self, b: &Vec3) -> f32 {
        unsafe { graphene_sys::graphene_vec3_dot(self.to_glib_none().0, b.to_glib_none().0) }
    }

    fn equal(&self, v2: &Vec3) -> bool {
        unsafe {
            from_glib(graphene_sys::graphene_vec3_equal(
                self.to_glib_none().0,
                v2.to_glib_none().0,
            ))
        }
    }

    pub fn get_x(&self) -> f32 {
        unsafe { graphene_sys::graphene_vec3_get_x(self.to_glib_none().0) }
    }

    pub fn get_xy(&self) -> Vec2 {
        unsafe {
            let mut res = Vec2::uninitialized();
            graphene_sys::graphene_vec3_get_xy(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn get_xy0(&self) -> Vec3 {
        unsafe {
            let mut res = Vec3::uninitialized();
            graphene_sys::graphene_vec3_get_xy0(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn get_xyz0(&self) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            graphene_sys::graphene_vec3_get_xyz0(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn get_xyz1(&self) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            graphene_sys::graphene_vec3_get_xyz1(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn get_xyzw(&self, w: f32) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            graphene_sys::graphene_vec3_get_xyzw(
                self.to_glib_none().0,
                w,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn get_y(&self) -> f32 {
        unsafe { graphene_sys::graphene_vec3_get_y(self.to_glib_none().0) }
    }

    pub fn get_z(&self) -> f32 {
        unsafe { graphene_sys::graphene_vec3_get_z(self.to_glib_none().0) }
    }

    pub fn init(&mut self, x: f32, y: f32, z: f32) {
        unsafe {
            graphene_sys::graphene_vec3_init(self.to_glib_none_mut().0, x, y, z);
        }
    }

    //pub fn init_from_float(&mut self, src: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 3) -> Option<Vec3> {
    //    unsafe { TODO: call graphene_sys:graphene_vec3_init_from_float() }
    //}

    pub fn init_from_vec3(&mut self, src: &Vec3) {
        unsafe {
            graphene_sys::graphene_vec3_init_from_vec3(
                self.to_glib_none_mut().0,
                src.to_glib_none().0,
            );
        }
    }

    pub fn length(&self) -> f32 {
        unsafe { graphene_sys::graphene_vec3_length(self.to_glib_none().0) }
    }

    pub fn max(&self, b: &Vec3) -> Vec3 {
        unsafe {
            let mut res = Vec3::uninitialized();
            graphene_sys::graphene_vec3_max(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn min(&self, b: &Vec3) -> Vec3 {
        unsafe {
            let mut res = Vec3::uninitialized();
            graphene_sys::graphene_vec3_min(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn multiply(&self, b: &Vec3) -> Vec3 {
        unsafe {
            let mut res = Vec3::uninitialized();
            graphene_sys::graphene_vec3_multiply(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn near(&self, v2: &Vec3, epsilon: f32) -> bool {
        unsafe {
            from_glib(graphene_sys::graphene_vec3_near(
                self.to_glib_none().0,
                v2.to_glib_none().0,
                epsilon,
            ))
        }
    }

    pub fn negate(&self) -> Vec3 {
        unsafe {
            let mut res = Vec3::uninitialized();
            graphene_sys::graphene_vec3_negate(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn normalize(&self) -> Vec3 {
        unsafe {
            let mut res = Vec3::uninitialized();
            graphene_sys::graphene_vec3_normalize(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn scale(&self, factor: f32) -> Vec3 {
        unsafe {
            let mut res = Vec3::uninitialized();
            graphene_sys::graphene_vec3_scale(
                self.to_glib_none().0,
                factor,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn subtract(&self, b: &Vec3) -> Vec3 {
        unsafe {
            let mut res = Vec3::uninitialized();
            graphene_sys::graphene_vec3_subtract(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    //pub fn to_float(&self, dest: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 3) {
    //    unsafe { TODO: call graphene_sys:graphene_vec3_to_float() }
    //}

    pub fn one() -> Vec3 {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(graphene_sys::graphene_vec3_one()) }
    }

    pub fn x_axis() -> Vec3 {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(graphene_sys::graphene_vec3_x_axis()) }
    }

    pub fn y_axis() -> Vec3 {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(graphene_sys::graphene_vec3_y_axis()) }
    }

    pub fn z_axis() -> Vec3 {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(graphene_sys::graphene_vec3_z_axis()) }
    }

    pub fn zero() -> Vec3 {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(graphene_sys::graphene_vec3_zero()) }
    }
}

impl PartialEq for Vec3 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Vec3 {}

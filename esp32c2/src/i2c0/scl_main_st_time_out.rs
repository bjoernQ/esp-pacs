#[doc = "Register `SCL_MAIN_ST_TIME_OUT` reader"]
pub type R = crate::R<SCL_MAIN_ST_TIME_OUT_SPEC>;
#[doc = "Register `SCL_MAIN_ST_TIME_OUT` writer"]
pub type W = crate::W<SCL_MAIN_ST_TIME_OUT_SPEC>;
#[doc = "Field `SCL_MAIN_ST_TO_I2C` reader - The threshold value of SCL_MAIN_FSM state unchanged period.nIt should be o more than 23"]
pub type SCL_MAIN_ST_TO_I2C_R = crate::FieldReader;
#[doc = "Field `SCL_MAIN_ST_TO_I2C` writer - The threshold value of SCL_MAIN_FSM state unchanged period.nIt should be o more than 23"]
pub type SCL_MAIN_ST_TO_I2C_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - The threshold value of SCL_MAIN_FSM state unchanged period.nIt should be o more than 23"]
    #[inline(always)]
    pub fn scl_main_st_to_i2c(&self) -> SCL_MAIN_ST_TO_I2C_R {
        SCL_MAIN_ST_TO_I2C_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_MAIN_ST_TIME_OUT")
            .field(
                "scl_main_st_to_i2c",
                &format_args!("{}", self.scl_main_st_to_i2c().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCL_MAIN_ST_TIME_OUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - The threshold value of SCL_MAIN_FSM state unchanged period.nIt should be o more than 23"]
    #[inline(always)]
    #[must_use]
    pub fn scl_main_st_to_i2c(&mut self) -> SCL_MAIN_ST_TO_I2C_W<SCL_MAIN_ST_TIME_OUT_SPEC, 0> {
        SCL_MAIN_ST_TO_I2C_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SCL main status time out register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_main_st_time_out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_main_st_time_out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_MAIN_ST_TIME_OUT_SPEC;
impl crate::RegisterSpec for SCL_MAIN_ST_TIME_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_main_st_time_out::R`](R) reader structure"]
impl crate::Readable for SCL_MAIN_ST_TIME_OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_main_st_time_out::W`](W) writer structure"]
impl crate::Writable for SCL_MAIN_ST_TIME_OUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCL_MAIN_ST_TIME_OUT to value 0x10"]
impl crate::Resettable for SCL_MAIN_ST_TIME_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}

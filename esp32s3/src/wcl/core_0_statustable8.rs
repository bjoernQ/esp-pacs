#[doc = "Register `Core_0_STATUSTABLE8` reader"]
pub type R = crate::R<CORE_0_STATUSTABLE8_SPEC>;
#[doc = "Register `Core_0_STATUSTABLE8` writer"]
pub type W = crate::W<CORE_0_STATUSTABLE8_SPEC>;
#[doc = "Field `CORE_0_FROM_WORLD_8` reader - This bit is used to confirm world before enter entry 8"]
pub type CORE_0_FROM_WORLD_8_R = crate::BitReader;
#[doc = "Field `CORE_0_FROM_WORLD_8` writer - This bit is used to confirm world before enter entry 8"]
pub type CORE_0_FROM_WORLD_8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CORE_0_FROM_ENTRY_8` reader - This filed is used to confirm in which entry before enter entry 8"]
pub type CORE_0_FROM_ENTRY_8_R = crate::FieldReader;
#[doc = "Field `CORE_0_FROM_ENTRY_8` writer - This filed is used to confirm in which entry before enter entry 8"]
pub type CORE_0_FROM_ENTRY_8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CORE_0_CURRENT_8` reader - This bit is used to confirm whether the current state is in entry 8"]
pub type CORE_0_CURRENT_8_R = crate::BitReader;
#[doc = "Field `CORE_0_CURRENT_8` writer - This bit is used to confirm whether the current state is in entry 8"]
pub type CORE_0_CURRENT_8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 8"]
    #[inline(always)]
    pub fn core_0_from_world_8(&self) -> CORE_0_FROM_WORLD_8_R {
        CORE_0_FROM_WORLD_8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 8"]
    #[inline(always)]
    pub fn core_0_from_entry_8(&self) -> CORE_0_FROM_ENTRY_8_R {
        CORE_0_FROM_ENTRY_8_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 8"]
    #[inline(always)]
    pub fn core_0_current_8(&self) -> CORE_0_CURRENT_8_R {
        CORE_0_CURRENT_8_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_0_STATUSTABLE8")
            .field(
                "core_0_from_world_8",
                &format_args!("{}", self.core_0_from_world_8().bit()),
            )
            .field(
                "core_0_from_entry_8",
                &format_args!("{}", self.core_0_from_entry_8().bits()),
            )
            .field(
                "core_0_current_8",
                &format_args!("{}", self.core_0_current_8().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_STATUSTABLE8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 8"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_from_world_8(&mut self) -> CORE_0_FROM_WORLD_8_W<CORE_0_STATUSTABLE8_SPEC, 0> {
        CORE_0_FROM_WORLD_8_W::new(self)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 8"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_from_entry_8(&mut self) -> CORE_0_FROM_ENTRY_8_W<CORE_0_STATUSTABLE8_SPEC, 1> {
        CORE_0_FROM_ENTRY_8_W::new(self)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 8"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_current_8(&mut self) -> CORE_0_CURRENT_8_W<CORE_0_STATUSTABLE8_SPEC, 5> {
        CORE_0_CURRENT_8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Status register of world switch of entry 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_statustable8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_statustable8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_STATUSTABLE8_SPEC;
impl crate::RegisterSpec for CORE_0_STATUSTABLE8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_statustable8::R`](R) reader structure"]
impl crate::Readable for CORE_0_STATUSTABLE8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_statustable8::W`](W) writer structure"]
impl crate::Writable for CORE_0_STATUSTABLE8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_0_STATUSTABLE8 to value 0"]
impl crate::Resettable for CORE_0_STATUSTABLE8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

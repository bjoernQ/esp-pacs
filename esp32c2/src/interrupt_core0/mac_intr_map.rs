#[doc = "Register `MAC_INTR_MAP` reader"]
pub type R = crate::R<MAC_INTR_MAP_SPEC>;
#[doc = "Register `MAC_INTR_MAP` writer"]
pub type W = crate::W<MAC_INTR_MAP_SPEC>;
#[doc = "Field `WIFI_MAC_INT_MAP` reader - Need add description"]
pub type WIFI_MAC_INT_MAP_R = crate::FieldReader;
#[doc = "Field `WIFI_MAC_INT_MAP` writer - Need add description"]
pub type WIFI_MAC_INT_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn wifi_mac_int_map(&self) -> WIFI_MAC_INT_MAP_R {
        WIFI_MAC_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAC_INTR_MAP")
            .field(
                "wifi_mac_int_map",
                &format_args!("{}", self.wifi_mac_int_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MAC_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_mac_int_map(&mut self) -> WIFI_MAC_INT_MAP_W<MAC_INTR_MAP_SPEC, 0> {
        WIFI_MAC_INT_MAP_W::new(self)
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
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_INTR_MAP_SPEC;
impl crate::RegisterSpec for MAC_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_intr_map::R`](R) reader structure"]
impl crate::Readable for MAC_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_intr_map::W`](W) writer structure"]
impl crate::Writable for MAC_INTR_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_INTR_MAP to value 0"]
impl crate::Resettable for MAC_INTR_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

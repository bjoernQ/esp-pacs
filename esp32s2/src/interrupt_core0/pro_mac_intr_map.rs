#[doc = "Register `PRO_MAC_INTR_MAP` reader"]
pub type R = crate::R<PRO_MAC_INTR_MAP_SPEC>;
#[doc = "Register `PRO_MAC_INTR_MAP` writer"]
pub type W = crate::W<PRO_MAC_INTR_MAP_SPEC>;
#[doc = "Field `PRO_MAC_INTR_MAP` reader - This register is used to map MAC_INTR interrupt signal to one of the CPU interrupts."]
pub type PRO_MAC_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `PRO_MAC_INTR_MAP` writer - This register is used to map MAC_INTR interrupt signal to one of the CPU interrupts."]
pub type PRO_MAC_INTR_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - This register is used to map MAC_INTR interrupt signal to one of the CPU interrupts."]
    #[inline(always)]
    pub fn pro_mac_intr_map(&self) -> PRO_MAC_INTR_MAP_R {
        PRO_MAC_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_MAC_INTR_MAP")
            .field(
                "pro_mac_intr_map",
                &format_args!("{}", self.pro_mac_intr_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_MAC_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register is used to map MAC_INTR interrupt signal to one of the CPU interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn pro_mac_intr_map(&mut self) -> PRO_MAC_INTR_MAP_W<PRO_MAC_INTR_MAP_SPEC, 0> {
        PRO_MAC_INTR_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MAC_INTR interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_mac_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_mac_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_MAC_INTR_MAP_SPEC;
impl crate::RegisterSpec for PRO_MAC_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_mac_intr_map::R`](R) reader structure"]
impl crate::Readable for PRO_MAC_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_mac_intr_map::W`](W) writer structure"]
impl crate::Writable for PRO_MAC_INTR_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_MAC_INTR_MAP to value 0x10"]
impl crate::Resettable for PRO_MAC_INTR_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
